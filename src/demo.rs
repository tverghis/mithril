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
        // Read the first 8 bytes and ensure they match the magic header.
        let mut header_buf = [0; 8];
        r.read_exact(&mut header_buf)?;

        if header_buf != Self::SOURCE2_MAGIC_BYTES {
            return Err(MithrilErrorKind::InvalidDemo);
        }

        // Skip the next 8 bytes.
        r.read_exact(&mut [0; 8])?;

        // The remaining bytes constitute the actual demo data.
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        let demo = Self { bytes: buf };

        Ok(demo)
    }

    /// A read-only view into the inner data.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from_bytes {
        use super::*;

        #[test]
        #[should_panic]
        fn fails_for_empty_demo() {
            let bytes = vec![];
            let _ = Demo::try_from_read(&mut bytes.as_slice()).unwrap();
        }

        #[test]
        #[should_panic]
        fn fails_for_invalid_demo() {
            let bytes = vec![0xFF, 0xFF];
            let _ = Demo::try_from_read(&mut bytes.as_slice()).unwrap();
        }

        #[test]
        #[should_panic]
        fn fails_for_demo_with_insufficient_bytes() {
            let mut bytes = vec![];
            bytes.extend(Demo::SOURCE2_MAGIC_BYTES);
            bytes.extend([0x00, 0x00]); // Not enough bytes in the unused section, reading should fail.

            let _ = Demo::try_from_read(&mut bytes.as_slice()).unwrap();
        }

        #[test]
        fn works_for_valid_demo() {
            let unused_bytes = [0xDE, 0xAD, 0xC0, 0xDE, 0xDE, 0xAD, 0xC0, 0xDE];
            let data = [0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF];
            let bytes = [Demo::SOURCE2_MAGIC_BYTES, unused_bytes, data].concat();

            let demo = Demo::try_from_read(&mut bytes.as_slice()).unwrap();

            assert_eq!(demo.bytes(), data)
        }
    }
}
