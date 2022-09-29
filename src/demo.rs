use crate::error::MithrilErrorKind;

use std::io::Read;

/// A `Demo` represents the data in a `.dem` file.
/// All associated functions that construct a `Demo` (including the implemention of `TryFrom<Vec<u8>>`)
/// enforce that the `.dem` file is a valid Source 2 demo.
#[derive(Debug)]
pub struct Demo {
    bytes: Vec<u8>,
}

impl Demo {
    // All valid Dota 2 replays (using the Source 2 engine) start with these magic 8 bytes.
    const SOURCE2_MAGIC_BYTES: [u8; 8] = [b'P', b'B', b'D', b'E', b'M', b'S', b'2', 0x00];

    /// Construct a new `Demo` from anything that implements [std::io::Read].
    /// This can fail, for example, if the bytes do not start with the magic header.
    pub fn try_from_read<R: Read>(r: &mut R) -> Result<Self, MithrilErrorKind> {
        // TODO: Check if the demo is valid first before reading the whole thing into memory.

        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        Self::try_from(buf)
    }

    /// A read-only view into the inner data.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Try to create a `Demo` from arbitrary bytes.
/// This can fail, for example, if the bytes do not start with the magic header.
impl TryFrom<Vec<u8>> for Demo {
    type Error = MithrilErrorKind;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if !is_valid_demo(&bytes) {
            return Err(MithrilErrorKind::InvalidDemo);
        }

        let replay = Self { bytes };

        Ok(replay)
    }
}

// Checks that the demo starts with the expected magic header bytes.
fn is_valid_demo(demo_bytes: &[u8]) -> bool {
    demo_bytes.starts_with(&Demo::SOURCE2_MAGIC_BYTES)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validate {
        use super::*;

        #[test]
        fn empty_demo_is_invalid() {
            assert!(!is_valid_demo(&[]));
        }

        #[test]
        fn insufficient_magic_bytes_is_invalid_demo() {
            assert!(!is_valid_demo(&[b'P', b'B', b'D']));
        }

        #[test]
        fn incorrect_magic_bytes_is_invalid_demo() {
            assert!(!is_valid_demo(&[0xFF; 8]));
        }
    }

    mod try_from_bytes {
        use super::*;

        #[test]
        #[should_panic]
        fn fails_for_invalid_demo() {
            let bytes = vec![0xFF, 0xFF];
            let _ = Demo::try_from(bytes).unwrap();
        }

        #[test]
        fn works_for_valid_demo() {
            let data = [0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF];
            let bytes = [Demo::SOURCE2_MAGIC_BYTES, data].concat();

            let demo = Demo::try_from(bytes).unwrap();

            assert_eq!(
                demo.bytes().len(),
                Demo::SOURCE2_MAGIC_BYTES.len() + data.len()
            )
        }
    }
}
