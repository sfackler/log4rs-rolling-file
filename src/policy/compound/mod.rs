//! The compound rolling policy.
use log4rs::file::{Deserialize, Deserializers};
use serde_value::Value;
use std::error::Error;

use LogFile;
use policy::compound::config::Config;
use policy::compound::roll::Roll;
use policy::compound::trigger::Trigger;
use policy::Policy;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod config;
pub mod roll;
pub mod trigger;

/// A rolling policy which delegates to a "trigger" and "roller".
///
/// The trigger determines if the log file should roll, for example, by checking
/// the size of the file. The roller processes the old log file, for example,
/// by compressing it and moving it to a different location.
#[derive(Debug)]
pub struct CompoundPolicy {
    trigger: Box<Trigger>,
    roller: Box<Roll>,
}

impl CompoundPolicy {
    /// Creates a new `CompoundPolicy`.
    pub fn new(trigger: Box<Trigger>, roller: Box<Roll>) -> CompoundPolicy {
        CompoundPolicy {
            trigger: trigger,
            roller: roller,
        }
    }
}

impl Policy for CompoundPolicy {
    fn process(&self, log: &mut LogFile) -> Result<(), Box<Error>> {
        if try!(self.trigger.trigger(log)) {
            log.roll();
            try!(self.roller.roll(log.path()))
        }
        Ok(())
    }
}

/// A deserializer for the `CompoundPolicyDeserializer`.
///
/// # Configuration
///
/// ```yaml
/// kind: compound
///
/// # The trigger, which determines when the log will roll over. Required.
/// trigger:
///
///   # Identifies which trigger is to be used. Required.
///   kind: size
///
///   # The remainder of the configuration is passed to the trigger's
///   # deserializer, and will vary based on the kind of trigger.
///   limit: 10 mb
///
/// # The roller, which processes the old log file. Required.
/// roller:
///
///   # Identifies which roller is to be used. Required.
///   kind: delete
///
///   # The remainder of the configuration is passed to the roller's
///   # deserializer, and will vary based on the kind of roller.
/// ```
pub struct CompoundPolicyDeserializer;

impl Deserialize for CompoundPolicyDeserializer {
    type Trait = Policy;

    fn deserialize(&self,
                   config: Value,
                   deserializers: &Deserializers)
                   -> Result<Box<Policy>, Box<Error>> {
        let config: Config = try!(config.deserialize_into());
        let trigger =
            try!(deserializers.deserialize("trigger", &config.trigger.kind, config.trigger.config));
        let roller =
            try!(deserializers.deserialize("roller", &config.roller.kind, config.roller.config));
        Ok(Box::new(CompoundPolicy::new(trigger, roller)))
    }
}
