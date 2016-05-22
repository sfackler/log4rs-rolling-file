use log4rs::file::{Deserialize, Deserializers};
use serde_value::Value;
use std::error::Error;

use trigger::{LogFile, Trigger};
use trigger::size::config::Config;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

#[derive(Debug)]
pub struct SizeTrigger {
    limit: u64,
}

impl SizeTrigger {
    pub fn new(limit: u64) -> SizeTrigger {
        SizeTrigger { limit: limit }
    }
}

impl Trigger for SizeTrigger {
    fn trigger(&self, file: &LogFile) -> Result<bool, Box<Error>> {
        Ok(file.len() > self.limit)
    }
}

pub struct SizeTriggerDeserializer;

impl Deserialize for SizeTriggerDeserializer {
    type Trait = Trigger;

    fn deserialize(&self, config: Value, _: &Deserializers) -> Result<Box<Trigger>, Box<Error>> {
        let config: Config = try!(config.deserialize_into());
        Ok(Box::new(SizeTrigger::new(config.limit)))
    }
}
