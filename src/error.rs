/// Represents an error that may occur during the loading or parsing of the demo data.
#[derive(Debug)]
pub enum MithrilErrorKind {
    /// A wrapped [std::io::Error] that is typically encountered when loading the demo data into memory
    IoError(std::io::Error),
    /// The demo did not start with the recognized magic bytes
    InvalidDemo,
}

impl From<std::io::Error> for MithrilErrorKind {
    fn from(e: std::io::Error) -> Self {
        MithrilErrorKind::IoError(e)
    }
}
