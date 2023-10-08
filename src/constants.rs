pub const FRAME_END: u8 = 206; // xCE

pub const PROTOCOL_HEADER: [u8; 8] = [65, 77, 81, 80, 0, 0, 9, 1]; // "AMQP0091"

pub mod size {
    pub const FIELD_TYPE_SIZE: usize = 1;
    pub const U8_SIZE: usize = 1;
    pub const U16_SIZE: usize = 2;
    pub const U32_SIZE: usize = 4;
    pub const BOOL_SIZE: usize = 1;
    pub const HEADER_SIZE: usize = 7;
    pub const CLASS_OR_METHOD_TYPE_SIZE: usize = 2;
}

pub mod field_type {
    pub const BOOL: char = 't';
    pub const LONG_STRING: char = 'S';
    pub const SHORT_STRING: char = 's';
    pub const TABLE: char = 'F';
}
