// Joseph Prichard
// 1/5/2023
// Bytewise and bitwise utilities for symbols to be compressed

#[derive(Clone, Copy)]
pub struct SymbolCode {
    pub symbol: u8,
    pub code: u32,
    pub bit_len: u8
}

impl SymbolCode {
    pub fn append_bit(&self, bit: u32) -> SymbolCode {
        SymbolCode {
            symbol: self.symbol,
            code: self.code ^ (bit << self.bit_len),
            bit_len: self.bit_len + 1
        }
    }

}

pub fn get_bit(byte: u8, n: u32) -> u8 {
    ((byte >> n) & 1) as u8
}
