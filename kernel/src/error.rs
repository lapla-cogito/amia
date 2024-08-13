#[derive(Debug)]
pub enum Error {
    IoErr,
    MutexErr(&'static str),
    OutOfMemory,
    InvalidElf,
}

pub type Result<T> = core::result::Result<T, Error>;
