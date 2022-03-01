use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum AMError {
    IO,
    Int,
    Poison,
    TODO(u8),
    FS(AMErrorFS),
    Uninit,
}

impl std::fmt::Display for AMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AMError: {:?}",self) // user-facing output
    }
}

impl Error for AMError {}

impl From<u8> for AMError {
    fn from(_: u8) -> Self {
        panic!();
    }
}

impl From<std::io::Error> for AMError {
    fn from(_: std::io::Error) -> Self {
        AMError::IO
    }
}

impl From<std::num::TryFromIntError> for AMError {
    fn from(_: std::num::TryFromIntError) -> Self {
        AMError::Int
    }
}

impl From<AMErrorFS> for AMError {
    fn from(e: AMErrorFS) -> Self {
        AMError::FS(e)
    }
}

impl std::fmt::Display for AMErrorFS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FSError: {:?}",self) // user-facing output
    }
}

impl Error for AMErrorFS {}

#[derive(Debug, PartialEq)]
pub enum AMErrorFS {
    NoSuperblock,
    NoRootgroup,
    Checksum,
    Signature,
    DiskID,
    NullPointer,
    UnknownDevid,
}
