// Joseph Prichard
// 1/5/2023
// File reading and writing utilities

use std::error;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::bitwise::get_bit;

const BUFFER_LEN: usize = 512;
const BUFFER_BIT_LEN: u32 = 512 * 8;

pub struct BitFileStream {
    // the file stream to read and write to
    file: File,
    // a buffer storing a block from the file
    buffer: [u8; BUFFER_LEN],
    // the number of bytes read from the file into the buffer
    read_size: usize,
    // the bit position of the last read in the buffer
    bit_position: u32
}

impl BitFileStream {
    pub fn new(filepath: &str) -> Result<BitFileStream, Box<dyn Error>> {
        // open the file into memory
        let mut file = File::open(filepath)?;
        // read the first buffer into memory
        let mut buffer = [0u8; BUFFER_LEN];
        let read_size = file.read(&mut buffer)?;
        // copy necessary resources into the struct
        Ok(BitFileStream {
            file,
            buffer,
            read_size,
            bit_position: 0
        })
    }

    pub fn read_byte(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        // if buffer pointer goes past read size, we're at end of file
        if self.bit_position > (8 * self.read_size) as u32 {
            return Ok(None);
        }

        // read a new buffer
        if self.bit_position > BUFFER_BIT_LEN {
            self.read_size = self.file.read(&mut self.buffer)?;
            self.bit_position = 0;
            if self.read_size == 0 {
                return Ok(None);
            }
        }

        // read the byte the current bit stored is at
        Ok(Some(self.buffer[(self.bit_position / 8) as usize]))
    }

    pub fn consume_byte(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        if let Some(byte) = self.read_byte()? {
            self.bit_position += 8;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    pub fn consume_bit(&mut self) -> Result<Option<u8>, Box<dyn Error>> {
        if let Some(byte) = self.read_byte()? {
            let bit = get_bit(byte, self.bit_position % 8);
            self.bit_position += 1;
            Ok(Some(bit))
        } else {
            Ok(None)
        }
    }

    pub fn write_bit(&mut self, bit: u8) {

    }
}

