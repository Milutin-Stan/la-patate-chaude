use std::fmt;
use serde::{Serialize, Deserialize};
use crate::version::Version;
#[derive(Debug, Serialize, Deserialize)]
pub enum Welcome {
    Welcome(Version),
}


