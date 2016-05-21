extern crate antidote;
extern crate log;
extern crate log4rs;
extern crate serde;

use antidote::Mutex;
use log4rs::append::Append;
use log4rs::encode::{self, Encode};
use log::LogRecord;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};

use roll::Roll;
use trigger::{LogFile, Trigger};

pub mod roll;
pub mod trigger;

struct LogWriter {
    file: BufWriter<File>,
    len: u64,
}

impl io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf).map(|n| { self.len += n as u64; n })
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

trait LogFileInternals<'a> {
    fn new(path: &'a Path, len: u64) -> LogFile<'a>;
}
