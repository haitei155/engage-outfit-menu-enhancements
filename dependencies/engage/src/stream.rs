use std::io;

use unity::prelude::*;

#[unity::class("App", "Stream")]
pub struct Stream {
    buffer: &'static mut Il2CppArray<u8>,
    position: i32,
    stack: *const u8 ,
}

#[unity::from_offset("App", "Stream", "WriteInt")]
extern "C" fn stream_write_int(this: &mut Stream, data: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Stream", "ReadInt")]
extern "C" fn stream_read_int(this: &mut Stream, method_info: OptionalMethod) -> i32;

impl Stream {
    pub fn get_position(&self) -> usize {
        self.position as _
    }

    /// /!\ To be deprecated in favor of a std::io::Write implementation in due time.
    /// 
    /// Write a i32 into the stream and returns how many bytes were written.
    /// An error of type WriteZero is returned if the buffer was too small to write the value.
    /// 
    /// While useless at the moment, this result type was chosen to facilitate migrating to proper Rust utilities later.
    pub fn write_int(&mut self, data: i32) -> io::Result<usize> {
        if self.buffer.len() >= self.get_position() + 4 {
            unsafe { stream_write_int(self, data, None) }
            Ok(4)
        } else {
            Err(io::Error::from(io::ErrorKind::WriteZero))
        }
    }
    
    /// /!\ To be deprecated in favor of a std::io::Read implementation in due time.
    /// 
    /// Read a i32 from the stream and returns the value.
    /// An error of type UnexpectedEof is returned if the buffer did not have enough bytes left to read the value.
    /// 
    /// While useless at the moment, this result type was chosen to facilitate migrating to proper Rust utilities later.
    pub fn read_int(&mut self) -> io::Result<i32> {
        if self.buffer.len() >= self.get_position() + 4 {
            Ok(unsafe { stream_read_int(self, None) })
        } else {
            Err(io::Error::from(io::ErrorKind::UnexpectedEof))
        }
    }
}