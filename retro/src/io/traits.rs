use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};

use log::debug;

pub type DynError = Box<dyn Error + Send + Sync + 'static>;

/// Can read, write, open and save.  Whether this is to a local disk, network storage or memory does not matter
pub trait ReadableIo {
    fn read(&self) -> Result<String, DynError>;

    fn read_u8(&self) -> Result<Vec<u8>, DynError>;
}

pub trait WritableIo {
    /// a way to get access to the IoResource (eg file path, URL address)
    type Path;

    fn write(&self, path: Self::Path) -> Result<(), IoResourceError>;
}

#[derive(Debug, Clone)]
pub struct IoResourceError {
    error: String,
}

impl Display for IoResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

impl Error for IoResourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl ReadableIo for String {
    fn read(&self) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let file = File::open(&self)?;
        let mut reader = BufReader::new(file);
        let mut buff = vec![];
        let result = reader.read_to_end(&mut buff)?;
        debug!("Read in {result} bytes");

        let body = std::str::from_utf8(&buff).map(|s| s.to_string())?;
        Ok(body)
    }

    fn read_u8(&self) -> Result<Vec<u8>, DynError> {
        let file = File::open(&self)?;
        let mut reader = BufReader::new(file);
        let mut buff = vec![];
        let result = reader.read_to_end(&mut buff)?;
        debug!("Read in {result} bytes");

        Ok(buff)
    }
}
