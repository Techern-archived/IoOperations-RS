//! Various I/O operations for Rust

pub mod reader;

pub mod writer;

#[cfg(test)]
mod test {

    use super::reader::Reader;
    use super::writer::Writer;

    use std::io::Cursor;

    #[test]
    fn test_read_write_u8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_u8(15).is_ok());
        assert!(vector.write_u8(62).is_ok());

        vector.set_position(0);

        assert_eq!(15, vector.read_u8().unwrap());
        assert_eq!(62, vector.read_u8().unwrap());

    }

    #[test]
    fn test_read_write_i8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_i8(-43).is_ok());
        assert!(vector.write_i8(33).is_ok());

        vector.set_position(0);

        assert_eq!(-43, vector.read_i8().unwrap());
        assert_eq!(33, vector.read_i8().unwrap());

    }

    #[test]
    fn test_read_write_be_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_u16(64241).is_ok());
        assert!(vector.write_be_u16(4).is_ok());

        vector.set_position(0);

        assert_eq!(64241, vector.read_be_u16().unwrap());
        assert_eq!(4, vector.read_be_u16().unwrap());

    }

    #[test]
    fn test_read_write_le_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_u16(64241).is_ok());
        assert!(vector.write_le_u16(4).is_ok());

        vector.set_position(0);

        assert_eq!(64241, vector.read_le_u16().unwrap());
        assert_eq!(4, vector.read_le_u16().unwrap());

    }

    #[test]
    fn test_read_write_be_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_i16(-12234).is_ok());
        assert!(vector.write_be_i16(24524).is_ok());

        vector.set_position(0);

        assert_eq!(-12234, vector.read_be_i16().unwrap());
        assert_eq!(24524, vector.read_be_i16().unwrap());

    }

    #[test]
    fn test_read_write_le_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_i16(-12234).is_ok());
        assert!(vector.write_le_i16(24524).is_ok());

        vector.set_position(0);

        assert_eq!(-12234, vector.read_le_i16().unwrap());
        assert_eq!(24524, vector.read_le_i16().unwrap());

    }

    #[test]
    fn test_read_write_be_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_u32(4000000000).is_ok());
        assert!(vector.write_be_u32(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(4000000000, vector.read_be_u32().unwrap());
        assert_eq!(26436735, vector.read_be_u32().unwrap());

    }

    #[test]
    fn test_read_write_le_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_u32(4000000000).is_ok());
        assert!(vector.write_le_u32(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(4000000000, vector.read_le_u32().unwrap());
        assert_eq!(26436735, vector.read_le_u32().unwrap());

    }

    #[test]
    fn test_read_write_be_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_i32(-2100000000).is_ok());
        assert!(vector.write_be_i32(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(-2100000000, vector.read_be_i32().unwrap());
        assert_eq!(26436735, vector.read_be_i32().unwrap());

    }

    #[test]
    fn test_read_write_le_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_i32(-2100000000).is_ok());
        assert!(vector.write_le_i32(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(-2100000000, vector.read_le_i32().unwrap());
        assert_eq!(26436735, vector.read_le_i32().unwrap());

    }

    #[test]
    fn test_read_write_be_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_u64(35754238465284).is_ok());
        assert!(vector.write_be_u64(423).is_ok());

        vector.set_position(0);

        assert_eq!(35754238465284, vector.read_be_u64().unwrap());
        assert_eq!(423, vector.read_be_u64().unwrap());

    }

    #[test]
    fn test_read_write_le_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_u64(35754238465284).is_ok());
        assert!(vector.write_le_u64(423).is_ok());

        vector.set_position(0);

        assert_eq!(35754238465284, vector.read_le_u64().unwrap());
        assert_eq!(423, vector.read_le_u64().unwrap());

    }

    #[test]
    fn test_read_write_be_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_be_i64(-5754238465284).is_ok());
        assert!(vector.write_be_i64(423).is_ok());

        vector.set_position(0);

        assert_eq!(-5754238465284, vector.read_be_i64().unwrap());
        assert_eq!(423, vector.read_be_i64().unwrap());

    }

    #[test]
    fn test_read_write_le_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_le_i64(-5754238465284).is_ok());
        assert!(vector.write_le_i64(423).is_ok());

        vector.set_position(0);

        assert_eq!(-5754238465284, vector.read_le_i64().unwrap());
        assert_eq!(423, vector.read_le_i64().unwrap());

    }

}
