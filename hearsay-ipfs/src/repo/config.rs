use std::sync::Arc;

use super::{RepoInner, Repository};

#[derive(Default)]
pub struct Config {}

impl Config {
    pub fn init(self) -> Repository {
        Repository { inner: Arc::new(RepoInner { capacity: 0.into() }) } // TODO: temp
    }
}
