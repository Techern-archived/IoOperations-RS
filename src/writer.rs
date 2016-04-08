//! Writing I/O operations

use std::io::Write;
use std::io::Result;

/// Extends the Write trait to provide common I/O writer operations
pub trait Writer : Write {

    /// Writes an unsigned byte to this Writer
    fn write_u8(&mut self, value: u8) -> Result<()> {

        let raw_buffer = vec![value];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)

    }

    /// Writes a signed byte to this Writer
    fn write_i8(&mut self, value: i8) -> Result<()> {

        let raw_buffer = vec![value as u8];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)

    }

    /// Writes an unsigned little-endian short to this Writer
    fn write_le_u16(&mut self, value: u16) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned big-endian short to this Writer
    fn write_be_u16(&mut self, value: u16) -> Result<()> {
        let raw_buffer = vec![
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed little-endian short to this Writer
    fn write_le_i16(&mut self, value: i16) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed big-endian short to this Writer
    fn write_be_i16(&mut self, value: i16) -> Result<()> {
        let raw_buffer = vec![
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned little-endian integer to this Writer
    fn write_le_u32(&mut self, value: u32) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8,
            (value >> 16) as u8,
            (value >> 24) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned big-endian integer to this Writer
    fn write_be_u32(&mut self, value: u32) -> Result<()> {
        let raw_buffer = vec![
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed little-endian integer to this Writer
    fn write_le_i32(&mut self, value: i32) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8,
            (value >> 16) as u8,
            (value >> 24) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed big-endian integer to this Writer
    fn write_be_i32(&mut self, value: i32) -> Result<()> {
        let raw_buffer = vec![
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned little-endian long to this Writer
    fn write_le_u64(&mut self, value: u64) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8,
            (value >> 16) as u8,
            (value >> 24) as u8,
            (value >> 32) as u8,
            (value >> 40) as u8,
            (value >> 48) as u8,
            (value >> 56) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed little-endian long to this Writer
    fn write_le_i64(&mut self, value: i64) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8,
            (value >> 16) as u8,
            (value >> 24) as u8,
            (value >> 32) as u8,
            (value >> 40) as u8,
            (value >> 48) as u8,
            (value >> 56) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned big-endian long to this Writer
    fn write_be_u64(&mut self, value: u64) -> Result<()> {
        let raw_buffer = vec![
            (value >> 56) as u8,
            (value >> 48) as u8,
            (value >> 40) as u8,
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed big-endian long to this Writer
    fn write_be_i64(&mut self, value: i64) -> Result<()> {
        let raw_buffer = vec![
            (value >> 56) as u8,
            (value >> 48) as u8,
            (value >> 40) as u8,
            (value >> 32) as u8,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

}

impl<T> Writer for T where T: Write { }
