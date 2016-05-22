use log4rs::file::{Deserialize, Deserializers};
use serde_value::Value;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

use roll::Roll;
use roll::fixed_window::config::Config;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

#[derive(Debug)]
pub struct FixedWindowRoller {
    pattern: String,
    base: u32,
    limit: u32,
}

impl FixedWindowRoller {
    pub fn builder() -> FixedWindowRollerBuilder {
        FixedWindowRollerBuilder {
            base: 0,
        }
    }
}

impl Roll for FixedWindowRoller {
    fn roll(&self, file: &Path) -> Result<(), Box<Error>> {
        for i in (self.base..self.limit + self.base).rev() {
            let src = self.pattern.replace("{}", &i.to_string());
            let dst = self.pattern.replace("{}", &(i + 1).to_string());
            try!(move_file(&src, &dst));
        }

        move_file(file, &self.pattern.replace("{}", "0")).map_err(Into::into)
    }
}

fn move_file<P>(src: P, dst: &str) -> io::Result<()>
    where P: AsRef<Path>
{
    // first try a rename
    match fs::rename(src.as_ref(), dst) {
        Ok(()) => return Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => {},
    }

    // fall back to a copy and delete if src and dst are on different mounts
    fs::copy(src.as_ref(), dst).and_then(|_| fs::remove_file(src.as_ref()))
}

pub struct FixedWindowRollerBuilder {
    base: u32,
}

impl FixedWindowRollerBuilder {
    pub fn base(mut self, base: u32) -> FixedWindowRollerBuilder {
        self.base = base;
        self
    }

    pub fn build(self, pattern: &str, limit: u32) -> Result<FixedWindowRoller, Box<Error>> {
        if !pattern.contains("{}") {
            return Err("pattern does not contain `{}`".into());
        }

        Ok(FixedWindowRoller {
            pattern: pattern.to_owned(),
            base: self.base,
            limit: limit,
        })
    }
}

pub struct FixedWindowRollerDeserializer;

impl Deserialize for FixedWindowRollerDeserializer {
    type Trait = Roll;

    fn deserialize(&self,
                   config: Value,
                   _: &Deserializers)
                   -> Result<Box<Roll>, Box<Error>> {
        let config: Config = try!(config.deserialize_into());
        let mut builder = FixedWindowRoller::builder();
        if let Some(base) = config.base {
            builder = builder.base(base);
        }

        Ok(Box::new(try!(builder.build(&config.pattern, config.limit))))
    }
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use std::fs::File;
    use std::io::{Read, Write};

    use roll::Roll;
    use super::*;

    #[test]
    fn rotation() {
        let dir = TempDir::new("rotation").unwrap();

        let base = dir.path().to_str().unwrap();
        let roller = FixedWindowRoller::builder()
                         .build(&format!("{}/foo.log.{{}}", base), 1)
                         .unwrap();

        let file = dir.path().join("foo.log");
        File::create(&file).unwrap().write_all(b"file1").unwrap();

        roller.roll(&file).unwrap();
        assert!(!file.exists());
        let mut contents = vec![];
        File::open(dir.path().join("foo.log.0")).unwrap().read_to_end(&mut contents).unwrap();
        assert_eq!(contents, b"file1");

        File::create(&file).unwrap().write_all(b"file2").unwrap();

        roller.roll(&file).unwrap();
        assert!(!file.exists());
        contents.clear();
        File::open(dir.path().join("foo.log.1")).unwrap().read_to_end(&mut contents).unwrap();
        assert_eq!(contents, b"file1");
        contents.clear();
        File::open(dir.path().join("foo.log.0")).unwrap().read_to_end(&mut contents).unwrap();
        assert_eq!(contents, b"file2");

        File::create(&file).unwrap().write_all(b"file3").unwrap();

        roller.roll(&file).unwrap();
        assert!(!file.exists());
        contents.clear();
        assert!(!dir.path().join("foo.log.2").exists());
        File::open(dir.path().join("foo.log.1")).unwrap().read_to_end(&mut contents).unwrap();
        assert_eq!(contents, b"file2");
        contents.clear();
        File::open(dir.path().join("foo.log.0")).unwrap().read_to_end(&mut contents).unwrap();
        assert_eq!(contents, b"file3");
    }
}
