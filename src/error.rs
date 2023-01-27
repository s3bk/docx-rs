use std::io::Error as IOError;
use std::error::Error;
use std::fmt::{self, Display};

use hard_xml::XmlError;
use zip::result::ZipError;

/// Error type of docx-rs
#[derive(Debug)]
pub enum DocxError {
    IO(IOError),
    Xml(XmlError),
    Zip(ZipError),
}
impl Display for DocxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DocxError::IO(ref e) => write!(f, "IO: {}", e),
            DocxError::Xml(ref e) => write!(f, "XML: {}", e),
            DocxError::Zip(ref e) => write!(f, "ZIP: {}", e),
        }
    }
}
impl Error for DocxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DocxError::IO(ref e) => Some(e),
            DocxError::Xml(ref e) => Some(e),
            DocxError::Zip(ref e) => Some(e),
        }
    }
}

impl From<IOError> for DocxError {
    fn from(err: IOError) -> Self {
        DocxError::IO(err)
    }
}

impl From<XmlError> for DocxError {
    fn from(err: XmlError) -> Self {
        DocxError::Xml(err)
    }
}

impl From<ZipError> for DocxError {
    fn from(err: ZipError) -> Self {
        DocxError::Zip(err)
    }
}

/// Specialized `Result` which the error value is `DocxError`.
pub type DocxResult<T> = Result<T, DocxError>;
