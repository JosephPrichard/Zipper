// Joseph Prichard
// 1/5/2023
// File reading and writing utilities

use std::error;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::bitwise;
use crate::bitwise::set_bit;

const BUFFER_LEN: usize = 512;
const BUFFER_BIT_LEN: u32 = (BUFFER_LEN * 8) as u32;

pub struct FileReader {
    // the file stream to read from
    file: File,
    // a buffer storing a block from the file
    buffer: [u8; BUFFER_LEN],
    // the number of bytes read from the file into the buffer
    read_size: usize,
    // the bit position of the last read in the buffer
    bit_position: u32
}

impl FileReader {
    pub fn new(filepath: &str) -> Result<FileReader, Box<dyn Error>> {
        // open the file into memory
        let mut file = File::open(filepath)?;
        // read the first buffer into memory
        let mut buffer = [0u8; BUFFER_LEN];
        let read_size = file.read(&mut buffer)?;
        // copy necessary resources into the struct
        Ok(FileReader {
            file,
            buffer,
            read_size,
            bit_position: 0
        })
    }

    fn load_byte(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        // if buffer pointer goes past read size, we're at end of file
        if self.bit_position > (8 * self.read_size) as u32 {
            return Ok(None);
        }

        // at end of buffer: read a new buffer
        if self.bit_position > BUFFER_BIT_LEN {
            self.read_size = self.file.read(&mut self.buffer)?;
            self.bit_position = 0;
            // new buffer is empty so file we're at end of file
            if self.read_size == 0 {
                return Ok(None);
            }
        }

        // read the byte the current bit stored is at
        Ok(Some(self.buffer[(self.bit_position / 8) as usize]))
    }

    pub fn read_byte(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        if let Some(byte) = self.load_byte()? {
            self.bit_position += 8;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    pub fn read_bit(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        if let Some(byte) = self.load_byte()? {
            let bit = bitwise::get_bit(byte as u32, self.bit_position % 8);
            self.bit_position += 1;
            Ok(Some(bit))
        } else {
            Ok(None)
        }
    }

    // alternative to read_byte that panics if a byte cannot be read
    pub fn force_read_byte(&mut self) -> Result<u8, Box<dyn Error>> {
        if let Some(byte) = self.read_byte()? {
            Ok(byte)
        } else {
            panic!("Failed to read byte: end of byte-stream");
        }
    }

    // alternative to read_bit that panics if a bit cannot be read
    pub fn force_read_bit(&mut self) -> Result<u8, Box<dyn Error>> {
        if let Some(bit) = self.read_bit()? {
            Ok(bit)
        } else {
            panic!("Failed to read bit: end of bit-stream");
        }
    }
}
