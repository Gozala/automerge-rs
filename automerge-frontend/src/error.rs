use std::error::Error;
use std::fmt;
use automerge_protocol::ObjectID;
use crate::mutation::Path;

#[derive(Debug, PartialEq)]
pub enum AutomergeFrontendError {
    InvalidChangeRequest,
    MissingObjectError(ObjectID),
    NoSuchPathError(Path),
    MismatchedSequenceNumber,
}

impl fmt::Display for AutomergeFrontendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AutomergeFrontendError {}

#[derive(Debug, PartialEq)]
pub enum InvalidInitialStateError {
    InitialStateMustBeMap
}

impl fmt::Display for InvalidInitialStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InvalidInitialStateError {}

