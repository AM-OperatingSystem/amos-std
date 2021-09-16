#[derive(Debug,PartialEq)]
pub enum AMError {
    IO(),
    Int(),
    TODO(u8),
    FS(AMErrorFS),
}

impl From<u8> for AMError {
    fn from(_:u8) -> Self {
        panic!();
    }
}

impl From<std::io::Error> for AMError {
    fn from(_:std::io::Error) -> Self {
        AMError::IO()
    }
}

impl From<std::num::TryFromIntError> for AMError {
    fn from(_:std::num::TryFromIntError) -> Self {
        AMError::Int()
    }
}

impl From<AMErrorFS> for AMError {
    fn from(e: AMErrorFS) -> Self {
        AMError::FS(e)
    }
}

#[derive(Debug,PartialEq)]
pub enum AMErrorFS {
    NoSuperblock,
    Checksum,
    Signature,
    DiskID,
}