//! A rolling file appender for log4rs.
//!
//! Logging directly to a file can be a dangerous proposition for long running
//! processes. You wouldn't want to start a server up and find out a couple
//! weeks later that the disk is filled with hundreds of gigabytes of logs! A
//! rolling file appender alleviates these issues by limiting the amount of log
//! data that's preserved.
//!
//! Like a normal file appender, a rolling file appender is configured with the
//! location of its log file and the encoder which formats log events written
//! to it. In addition, it holds "trigger" and "roller" objects. The trigger
//! determines when the current log file should roll over and be replaced with
//! a new one. The roller determines what happens to the old log file.
//!
//! For example, you may configure an appender to roll the log over once it
//! reaches 50 megabytes, and to preserve the last 10 log files.
#![doc(html_root_url="https://sfackler.github.io/log4rs-rolling-file/doc/v0.1.2")]
#![warn(missing_docs)]
extern crate antidote;
extern crate log;
extern crate log4rs;
extern crate serde;
extern crate serde_value;

#[cfg(feature = "gzip")]
extern crate flate2;

#[cfg(test)]
extern crate tempdir;

use antidote::Mutex;
use log4rs::append::Append;
use log4rs::encode::{self, Encode};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::file::{Deserialize, Deserializers};
use log::LogRecord;
use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use serde_value::Value;

use config::Config;
use policy::Policy;
use policy::compound::CompoundPolicyDeserializer;
use policy::compound::trigger::size::SizeTriggerDeserializer;
use policy::compound::roll::delete::DeleteRollerDeserializer;
use policy::compound::roll::fixed_window::FixedWindowRollerDeserializer;

pub mod policy;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

/// Registers this crate's deserializers.
///
/// The following mappings will be added:
///
/// * Appenders
///   * "rolling_file" -> `RollingFileAppenderDeserializer`
/// * Policies
///   *  "compound" -> `CompoundPolicyDeserializer`
/// * Triggers
///   * "size" -> `SizeTriggerDeserializer`
/// * Rollers
///   * "delete" -> `DeleteRollerDeserializer`
///   * "fixed_window" -> `FixedWindowRollerDeserializer`
pub fn register(d: &mut Deserializers) {
    d.insert("rolling_file".to_owned(),
             Box::new(RollingFileAppenderDeserializer));
    d.insert("compound".to_owned(),
             Box::new(CompoundPolicyDeserializer));
    d.insert("size".to_owned(), Box::new(SizeTriggerDeserializer));
    d.insert("delete".to_owned(), Box::new(DeleteRollerDeserializer));
    d.insert("fixed_window".to_owned(),
             Box::new(FixedWindowRollerDeserializer));
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

/// Information about the active log file.
pub struct LogFile<'a> {
    writer: &'a mut Option<LogWriter>,
    path: &'a Path,
    len: u64,
}

impl<'a> LogFile<'a> {
    /// Returns the path to the log file.
    pub fn path(&self) -> &Path {
        self.path
    }

    /// Returns an estimate of the log file's current size.
    ///
    /// This is calculated by taking the size of the log file when it is opened
    /// and adding the number of bytes written. It may be inaccurate if any
    /// writes have failed or if another process has modified the file
    /// concurrently.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Triggers the log file to roll over.
    ///
    /// A policy must call this method when it wishes to roll the log. The
    /// appender's handle to the file will be closed, which is necessary to
    /// move or delete the file on Windows.
    ///
    /// If this method is called, the log file must no longer be present when
    /// the policy returns.
    pub fn roll(&mut self) {
        *self.writer = None;
    }
}

/// An appender which archives log files in a configurable strategy.
pub struct RollingFileAppender {
    writer: Mutex<Option<LogWriter>>,
    path: PathBuf,
    append: bool,
    encoder: Box<Encode>,
    policy: Box<Policy>,
}

impl fmt::Debug for RollingFileAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RollingFileAppender")
           .field("path", &self.path)
           .field("append", &self.append)
           .field("encoder", &self.encoder)
           .field("policy", &self.policy)
           .finish()
    }
}

impl Append for RollingFileAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error>> {
        let mut writer = self.writer.lock();

        if writer.is_none() {
            if let Some(parent) = self.path.parent() {
                try!(fs::create_dir_all(parent));
            }

            let file = try!(OpenOptions::new()
                                .write(true)
                                .append(self.append)
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

        let mut file = LogFile {
            writer: &mut writer,
            path: &self.path,
            len: len,
        };

        self.policy.process(&mut file)
    }
}

impl RollingFileAppender {
    /// Creates a new `RollingFileAppenderBuilder`.
    pub fn builder() -> RollingFileAppenderBuilder {
        RollingFileAppenderBuilder {
            append: true,
            encoder: None,
        }
    }
}

/// A builder for the `RollingFileAppender`.
pub struct RollingFileAppenderBuilder {
    append: bool,
    encoder: Option<Box<Encode>>,
}

impl RollingFileAppenderBuilder {
    /// Determines if the appender will append to or truncate the log file.
    ///
    /// Defaults to `true`.
    pub fn append(mut self, append: bool) -> RollingFileAppenderBuilder {
        self.append = append;
        self
    }

    /// Sets the encoder used by the appender.
    ///
    /// Defaults to a `PatternEncoder` with the default pattern.
    pub fn encoder(mut self, encoder: Box<Encode>) -> RollingFileAppenderBuilder {
        self.encoder = Some(encoder);
        self
    }

    /// Constructs a `RollingFileAppender`.
    pub fn build<P>(self, path: P, policy: Box<Policy>) -> RollingFileAppender
        where P: AsRef<Path>
    {
        RollingFileAppender {
            writer: Mutex::new(None),
            path: path.as_ref().to_owned(),
            append: self.append,
            encoder: self.encoder.unwrap_or_else(|| Box::new(PatternEncoder::default())),
            policy: policy,
        }
    }
}

/// A deserializer for the `RollingFileAppender`.
///
/// # Configuration
///
/// ```yaml
/// kind: rolling_file
///
/// # The path of the log file. Required.
/// path: log/foo.log
///
/// # Specifies if the appender should append to or truncate the log file if it
/// # already exists. Defaults to `true`.
/// append: true
///
/// # The encoder to use to format output. Defaults to `kind: pattern`.
/// encoder:
///   kind: pattern
///
/// # The policy which handles rotation of the log file. Required.
/// policy:
///   # Identifies which policy is to be used. If no kind is specified, it will
///   # default to "compound".
///   kind: compound
///
///   # The remainder of the configuration is passed along to the policy's
///   # deserializer, and will vary based on the kind of policy.
///   trigger:
///     kind: size
///     limit: 10 mb
///
///   roller:
///     kind: delete
/// ```
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

        let policy = try!(deserializers.deserialize("policy",
                                                    &config.policy.kind,
                                                    config.policy.config));
        Ok(Box::new(builder.build(config.path, policy)))
    }
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
    path: foo.log
    policy:
      trigger:
        kind: size
        limit: 1024
      roller:
        kind: delete
  bar:
    kind: rolling_file
    path: foo.log
    policy:
      kind: compound
      trigger:
        kind: size
        limit: 5 mb
      roller:
        kind: fixed_window
        pattern: 'foo.log.{}'
        base: 1
        count: 5
";

        let mut deserializers = Deserializers::default();
        register(&mut deserializers);

        let config = Config::parse(config, Format::Yaml, &deserializers).unwrap();
        println!("{:?}", config.errors());
        assert!(config.errors().is_empty());
    }
}
