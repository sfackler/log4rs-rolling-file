//! The size trigger.

use log4rs::file::{Deserialize, Deserializers};
use serde_value::Value;
use std::error::Error;

use trigger::{LogFile, Trigger};
use trigger::size::config::Config;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

/// A trigger which rolls the log once it has passed a certain size.
#[derive(Debug)]
pub struct SizeTrigger {
    limit: u64,
}

impl SizeTrigger {
    /// Returns a new trigger which rolls the log once it has passed the
    /// specified size in bytes.
    pub fn new(limit: u64) -> SizeTrigger {
        SizeTrigger { limit: limit }
    }
}

impl Trigger for SizeTrigger {
    fn trigger(&self, file: &LogFile) -> Result<bool, Box<Error>> {
        Ok(file.len() > self.limit)
    }
}

/// A deserializer for the `SizeTrigger`.
///
/// # Configuration
///
/// ```yaml
/// kind: size
///
/// # The size limit in bytes. The following units are supported (case insensitive): 
/// # "b", "kb", "kib", "mb", "mib", "gb", "gib", "tb", "tib". The unit defaults to
/// # bytes if not specified. Required.
/// limit: 10 mb
/// ```
pub struct SizeTriggerDeserializer;

impl Deserialize for SizeTriggerDeserializer {
    type Trait = Trigger;

    fn deserialize(&self, config: Value, _: &Deserializers) -> Result<Box<Trigger>, Box<Error>> {
        let config: Config = try!(config.deserialize_into());
        Ok(Box::new(SizeTrigger::new(config.limit)))
    }
}
