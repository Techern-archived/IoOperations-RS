//! Reading I/O operations

use std::io::{Read, Error, ErrorKind};

/// Extends the Read trait to provide common I/O reader operations
pub trait Reader : Read {

    /// Reads an unsigned byte from this Reader
    fn read_unsigned_byte(&mut self) -> Result<u8, Error> {

        let mut raw_buffer = vec![0u8; 1];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 1 {
                    return Ok(raw_buffer[0]);
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read one byte"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed byte from this Reader
    fn read_signed_byte(&mut self) -> Result<i8, Error> {

        let mut raw_buffer = vec![0u8; 1];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 1 {
                    return Ok(raw_buffer[0] as i8);
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read one byte"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned big-endian short from this Reader
    fn read_unsigned_be_short(&mut self) -> Result<u16, Error> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        ((raw_buffer[0] as u16) << 8) |
                        raw_buffer[1] as u16
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read two bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned little-endian short from this Reader
    fn read_unsigned_le_short(&mut self) -> Result<u16, Error> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        raw_buffer[0] as u16 |
                        ((raw_buffer[1] as u16) << 8)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read two bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed big-endian short from this Reader
    fn read_signed_be_short(&mut self) -> Result<i16, Error> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        ((raw_buffer[0] as i16) << 8) |
                        raw_buffer[1] as i16
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read two bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed little-endian short from this Reader
    fn read_signed_le_short(&mut self) -> Result<i16, Error> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        raw_buffer[0] as i16 |
                        ((raw_buffer[1] as i16) << 8)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read two bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned big-endian integer from this Reader
    fn read_unsigned_be_int(&mut self) -> Result<u32, Error> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        ((raw_buffer[0] as u32) << 24) |
                        ((raw_buffer[1] as u32) << 16) |
                        ((raw_buffer[2] as u32) << 8) |
                        raw_buffer[3] as u32
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read four bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned little-endian integer from this Reader
    fn read_unsigned_le_int(&mut self) -> Result<u32, Error> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        raw_buffer[0] as u32 |
                        ((raw_buffer[1] as u32) << 8) |
                        ((raw_buffer[2] as u32) << 16) |
                        ((raw_buffer[3] as u32) << 24)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read four bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed big-endian integer from this Reader
    fn read_signed_be_int(&mut self) -> Result<i32, Error> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        ((raw_buffer[0] as i32) << 24) |
                        ((raw_buffer[1] as i32) << 16) |
                        ((raw_buffer[2] as i32) << 8) |
                        raw_buffer[3] as i32
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read four bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed little-endian integer from this Reader
    fn read_signed_le_int(&mut self) -> Result<i32, Error> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        raw_buffer[0] as i32 |
                        ((raw_buffer[1] as i32) << 8) |
                        ((raw_buffer[2] as i32) << 16) |
                        ((raw_buffer[3] as i32) << 24)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read four bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned big-endian long from this Reader
    fn read_unsigned_be_long(&mut self) -> Result<u64, Error> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        ((raw_buffer[0] as u64) << 56) |
                        ((raw_buffer[1] as u64) << 48) |
                        ((raw_buffer[2] as u64) << 40) |
                        ((raw_buffer[3] as u64) << 32) |
                        ((raw_buffer[4] as u64) << 24) |
                        ((raw_buffer[5] as u64) << 16) |
                        ((raw_buffer[6] as u64) << 8) |
                        raw_buffer[7] as u64
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read eight bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed big-endian long from this Reader
    fn read_signed_be_long(&mut self) -> Result<i64, Error> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        ((raw_buffer[0] as i64) << 56) |
                        ((raw_buffer[1] as i64) << 48) |
                        ((raw_buffer[2] as i64) << 40) |
                        ((raw_buffer[3] as i64) << 32) |
                        ((raw_buffer[4] as i64) << 24) |
                        ((raw_buffer[5] as i64) << 16) |
                        ((raw_buffer[6] as i64) << 8) |
                        raw_buffer[7] as i64
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read eight bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads an unsigned little-endian long from this Reader
    fn read_unsigned_le_long(&mut self) -> Result<u64, Error> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        raw_buffer[0] as u64 |
                        ((raw_buffer[1] as u64) << 8) |
                        ((raw_buffer[2] as u64) << 16) |
                        ((raw_buffer[3] as u64) << 24) |
                        ((raw_buffer[4] as u64) << 32) |
                        ((raw_buffer[5] as u64) << 40) |
                        ((raw_buffer[6] as u64) << 48) |
                        ((raw_buffer[7] as u64) << 56)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read eight bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }

    /// Reads a signed little-endian long from this Reader
    fn read_signed_le_long(&mut self) -> Result<i64, Error> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        raw_buffer[0] as i64 |
                        ((raw_buffer[1] as i64) << 8) |
                        ((raw_buffer[2] as i64) << 16) |
                        ((raw_buffer[3] as i64) << 24) |
                        ((raw_buffer[4] as i64) << 32) |
                        ((raw_buffer[5] as i64) << 40) |
                        ((raw_buffer[6] as i64) << 48) |
                        ((raw_buffer[7] as i64) << 56)
                    );
                } else {
                    return Err(Error::new(ErrorKind::Other, "Could not read eight bytes"));
                }
            },
            Err(error) => {
                return Err(error);
            }

        }

    }
}

impl Reader for ::std::io::Cursor<Vec<u8>> { }
impl Reader for ::std::net::TcpStream { }

#[cfg(test)]
mod tests {

    use super::Reader;

    use std::io::Cursor;

    #[test]
    fn test_read_empty_u8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_byte().is_err());

    }

    #[test]
    fn test_read_empty_i8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_byte().is_err());

    }

    #[test]
    fn test_read_empty_le_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_le_short().is_err());

    }

    #[test]
    fn test_read_empty_le_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_le_short().is_err());

    }

    #[test]
    fn test_read_empty_be_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_be_short().is_err());

    }

    #[test]
    fn test_read_empty_be_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_be_short().is_err());

    }

    #[test]
    fn test_read_empty_be_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_be_int().is_err());

    }

    #[test]
    fn test_read_empty_be_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_be_int().is_err());

    }

    #[test]
    fn test_read_empty_le_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_le_int().is_err());

    }

    #[test]
    fn test_read_empty_le_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_le_int().is_err());

    }

    #[test]
    fn test_read_empty_be_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_be_long().is_err());

    }

    #[test]
    fn test_read_empty_be_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_be_long().is_err());

    }

    #[test]
    fn test_read_empty_le_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_unsigned_le_long().is_err());

    }

    #[test]
    fn test_read_empty_le_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.read_signed_le_long().is_err());

    }

}
