//! Handles errors for the server type

use serde::{Deserialize, Serialize};


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Error {
    A
}