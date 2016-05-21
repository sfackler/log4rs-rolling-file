use trigger::{LogFile, Trigger};

#[derive(Debug)]
pub struct SizeTrigger {
    limit: u64,
}

impl SizeTrigger {
    pub fn new(limit: u64) -> SizeTrigger {
        SizeTrigger {
            limit: limit,
        }
    }
}

impl Trigger for SizeTrigger {
    fn trigger(&self, file: &LogFile) -> Result<bool, Box<Error>> {
        Ok(file.len() > self.limit)
    }
}
