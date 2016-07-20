//! The delete roller.

use log4rs::file::{Deserialize, Deserializers};
use serde_value::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

use policy::compound::roll::Roll;
use policy::compound::roll::delete::config::Config;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;

/// A roller which deletes the log file.
#[derive(Debug)]
pub struct DeleteRoller(());

impl Roll for DeleteRoller {
    fn roll(&self, file: &Path) -> Result<(), Box<Error>> {
        fs::remove_file(file).map_err(Into::into)
    }
}

impl DeleteRoller {
    /// Returns a new `DeleteRoller`.
    pub fn new() -> DeleteRoller {
        DeleteRoller(())
    }
}

/// A deserializer for the `DeleteRoller`.
///
/// # Configuration
///
/// ```yaml
/// kind: delete
/// ```
pub struct DeleteRollerDeserializer;

impl Deserialize for DeleteRollerDeserializer {
    type Trait = Roll;

    fn deserialize(&self, config: Value, _: &Deserializers) -> Result<Box<Roll>, Box<Error>> {
        let _: Config = try!(config.deserialize_into());
        Ok(Box::new(DeleteRoller::new()))
    }
}
