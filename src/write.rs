use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use crate::bitwise;
use crate::bitwise::{get_bit, SymbolCode};

const BUFFER_LEN: usize = 512;
const BUFFER_BIT_LEN: u32 = (BUFFER_LEN * 8) as u32;

pub struct FileWriter {
    // the file stream to write to
    file: File,
    // a buffer storing a block to be written to the file
    buffer: [u8; BUFFER_LEN],
    // the bit position of the last write in the buffer
    bit_position: u32
}

impl FileWriter {
    pub fn new(filepath: &str) -> Result<FileWriter, Box<dyn Error>> {
        Ok(FileWriter {
            file: OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(filepath)?,
            buffer: [0u8; BUFFER_LEN],
            bit_position: 0
        })
    }

    fn update_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        // check if at end of buffer: persist current buffer and start writing on a new one
        if self.bit_position > BUFFER_BIT_LEN {
            self.persist_buffer()?;
            self.bit_position = 0;
            self.buffer = [0u8; BUFFER_LEN];
        }
        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), Box<dyn Error>> {
        self.update_buffer()?;

        // write the byte into the buffer
        self.buffer[(self.bit_position / 8) as usize] = byte;
        self.bit_position += 8;

        Ok(())
    }

    pub fn write_bit(&mut self, bit: u8) -> Result<(), Box<dyn Error>> {
        self.update_buffer()?;

        // write the bit back into the buffer
        if bit > 0 {
            let i = (self.bit_position / 8) as usize;
            self.buffer[i] = bitwise::set_bit(self.buffer[i] as u32, self.bit_position % 8);
        }

        self.bit_position += 1;

        Ok(())
    }

    pub fn persist_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write(&self.buffer[0..((self.bit_position / 8) as usize)])?;
        Ok(())
    }

    pub fn write_symbol(&mut self, symbol: &SymbolCode) -> Result<(), Box<dyn Error>> {
        for i in 0..symbol.bit_len {
            self.write_bit(get_bit(symbol.encoded_symbol, i as u32))?;
        }
        Ok(())
    }

    pub fn write_number(&mut self, num: u32) {

    }
}