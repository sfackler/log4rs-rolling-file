extern crate antidote;
extern crate log;
extern crate log4rs;
extern crate serde;
extern crate serde_value;

use antidote::Mutex;
use log4rs::append::Append;
use log4rs::encode::{self, Encode};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::file::{Deserialize, Deserializers};
use log::LogRecord;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use serde_value::Value;

use config::Config;
use roll::Roll;
use trigger::{LogFile, Trigger};
use trigger::size::SizeTriggerDeserializer;
use roll::delete::DeleteRollerDeserializer;

pub mod roll;
pub mod trigger;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

/// Registers this crate's deserializers.
///
/// The following mappings will be added:
///
/// * Appenders
///   * "rolling_file" -> `RollingFileAppenderDeserializer`
/// * Triggers
///   * "size" -> `SizeTriggerDeserializer`
/// * Rollers
///   * "delete" -> `DeleteRollerDeserializer`
pub fn register(d: &mut Deserializers) {
    d.insert("rolling_file".to_owned(), Box::new(RollingFileAppenderDeserializer));
    d.insert("size".to_owned(), Box::new(SizeTriggerDeserializer));
    d.insert("delete".to_owned(), Box::new(DeleteRollerDeserializer));
}

struct LogWriter {
    file: BufWriter<File>,
    len: u64,
}

impl io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf).map(|n| {
            self.len += n as u64;
            n
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl encode::Write for LogWriter {}

pub struct RollingFileAppender {
    writer: Mutex<Option<LogWriter>>,
    path: PathBuf,
    append: bool,
    encoder: Box<Encode>,
    trigger: Box<Trigger>,
    roller: Box<Roll>,
}

impl fmt::Debug for RollingFileAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RollingFileAppender")
           .field("path", &self.path)
           .field("append", &self.append)
           .field("encoder", &self.encoder)
           .field("trigger", &self.trigger)
           .field("roller", &self.roller)
           .finish()
    }
}

impl Append for RollingFileAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error>> {
        let mut writer = self.writer.lock();

        if writer.is_none() {
            let file = try!(OpenOptions::new()
                                .write(true)
                                .append(true)
                                .truncate(!self.append)
                                .create(true)
                                .open(&self.path));
            let len = if self.append {
                try!(self.path.metadata()).len()
            } else {
                0
            };
            *writer = Some(LogWriter {
                file: BufWriter::with_capacity(1024, file),
                len: len,
            });
        }

        let len = {
            // :( unwrap
            let writer = writer.as_mut().unwrap();
            try!(self.encoder.encode(writer, record));
            try!(writer.flush());
            writer.len
        };

        if try!(self.trigger.trigger(&LogFile::new(&self.path, len))) {
            *writer = None;
            try!(self.roller.roll(&self.path));
        }

        Ok(())
    }
}

impl RollingFileAppender {
    pub fn builder() -> RollingFileAppenderBuilder {
        RollingFileAppenderBuilder {
            append: true,
            encoder: None,
        }
    }
}

pub struct RollingFileAppenderBuilder {
    append: bool,
    encoder: Option<Box<Encode>>,
}

impl RollingFileAppenderBuilder {
    pub fn append(mut self, append: bool) -> RollingFileAppenderBuilder {
        self.append = append;
        self
    }

    pub fn encoder(mut self, encoder: Box<Encode>) -> RollingFileAppenderBuilder {
        self.encoder = Some(encoder);
        self
    }

    pub fn build<P>(self, path: P, trigger: Box<Trigger>, roller: Box<Roll>) -> RollingFileAppender
        where P: AsRef<Path>
    {
        RollingFileAppender {
            writer: Mutex::new(None),
            path: path.as_ref().to_owned(),
            append: self.append,
            encoder: self.encoder.unwrap_or_else(|| Box::new(PatternEncoder::default())),
            trigger: trigger,
            roller: roller,
        }
    }
}

pub struct RollingFileAppenderDeserializer;

impl Deserialize for RollingFileAppenderDeserializer {
    type Trait = Append;

    fn deserialize(&self,
                   config: Value,
                   deserializers: &Deserializers)
                   -> Result<Box<Append>, Box<Error>> {
        let config: Config = try!(config.deserialize_into());

        let mut builder = RollingFileAppender::builder();
        if let Some(append) = config.append {
            builder = builder.append(append);
        }
        if let Some(encoder) = config.encoder {
            let encoder = try!(deserializers.deserialize("encoder", &encoder.kind, encoder.config));
            builder = builder.encoder(encoder);
        }

        let trigger = try!(deserializers.deserialize("trigger",
                                                     &config.trigger.kind,
                                                     config.trigger.config));
        let roller = try!(deserializers.deserialize("roller",
                                                    &config.roller.kind,
                                                    config.roller.config));
        Ok(Box::new(builder.build(config.path, trigger, roller)))
    }
}

trait LogFileInternals<'a> {
    fn new(path: &'a Path, len: u64) -> LogFile<'a>;
}

#[cfg(test)]
mod test {
    use log4rs::file::{Config, Deserializers, Format};

    use super::*;

    #[test]
    fn deserialize() {
        let config = "
appenders:
  foo:
    kind: rolling_file
    trigger:
      kind: size
      limit: 1024
    roller:
      kind: delete
loggers:
appenders:
";

        let mut deserializers = Deserializers::default();
        register(&mut deserializers);

        Config::parse(config, Format::Yaml, &deserializers).unwrap();
    }
}
