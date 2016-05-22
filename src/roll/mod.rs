use std::error::Error;
use std::fmt;
use std::path::Path;

pub mod delete;

pub trait Roll: fmt::Debug + Send + Sync + 'static {
    fn roll(&self, file: &Path) -> Result<(), Box<Error>>;
}
