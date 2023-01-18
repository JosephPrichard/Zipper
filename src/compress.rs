// Joseph Prichard
// 1/5/2023
// Byte-by-byte file compressor

use std::error;
use crate::{bitwise, coding};

pub fn compress_file(input_filepath: &str, output_filepath: &str) {
    // keep track of symbol counts
    let freq_table = coding::create_freq_table(input_filepath)
        .expect("Failed to create frequency table");
    // construct coding table
    let node = coding::create_code_tree(&freq_table)
        .expect("Failed to create code tree");
    // construct a symbol table
    let symbol_table = coding::create_code_table(node)
        .expect("Failed to create symbol table");
    // use symbol table to compress input file into output file
    do_compression(input_filepath, output_filepath, &symbol_table)
        .expect("Failed to compress file");
}

fn do_compression(input_filepath: &str, output_filepath: &str, symbol_table: &Vec<bitwise::SymbolCode>) -> Result<(), Box<dyn error::Error>> {
    Ok(())
}