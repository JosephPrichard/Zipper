// Joseph Prichard
// 1/5/2023
// Byte-by-byte file compressor

use std::collections::BinaryHeap;
use std::error;
use std::error::Error;
use crate::{bitwise};
use crate::debug::dump_binary_file;
use crate::tree::Node;
use crate::read::FileReader;
use crate::write::FileWriter;

pub const TABLE_SIZE: usize = 256;

pub fn compress_file(input_filepath: &str, output_filepath: &str) {
    let freq_table = create_freq_table(input_filepath).expect("Failed to create frequency table");
    let node = create_code_tree(&freq_table).expect("Failed to create code tree");
    let symbol_table = create_code_table(&node).expect("Failed to create symbol table");

    write_code_tree(output_filepath, &node).expect("Failed to compress file");
    dump_binary_file(output_filepath);

    write_compressed(input_filepath, output_filepath, &symbol_table).expect("Failed to compress file");
    dump_binary_file(output_filepath);
}

fn write_code_tree(output_filepath: &str, node: &Box<Node>) -> Result<(), Box<dyn Error>> {
    let mut writer = FileWriter::new(output_filepath)?;
    let tree_bit_len = 10 * 5 - 1;
    write_node(node, &mut writer)?;
    writer.persist_buffer()?;
    Ok(())
}

fn write_node(node: &Box<Node>, writer: &mut FileWriter) -> Result<(), Box<dyn Error>> {
    if node.is_root() {
        writer.write_bit(1)?;
        writer.write_byte(node.plain_symbol)?;
    } else {
        writer.write_bit(0)?;
        if let Some(left) = &node.left {
            write_node(left, writer)?;
        }
        if let Some(right) = &node.right {
            write_node(right, writer)?;
        }
    }
    Ok(())
}

fn write_compressed(input_filepath: &str, output_filepath: &str, symbol_table: &Vec<bitwise::SymbolCode>) -> Result<(), Box<dyn Error>> {
    let mut writer = FileWriter::new(output_filepath)?;
    let mut reader = FileReader::new(input_filepath)?;

    while let Some(byte) = reader.read_byte()? {
        writer.write_symbol(&symbol_table[byte as usize])?;
    }
    writer.persist_buffer()?;

    Ok(())
}

fn create_freq_table(input_filepath: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    // keep track of symbol counts
    let mut freq_table = vec![0u32; TABLE_SIZE];

    // iterate through each byte in the file
    let mut reader = FileReader::new(input_filepath)?;
    while let Some(byte) = reader.read_byte()? {
        freq_table[usize::from(byte)] += 1;
    }

    Ok(freq_table)
}

fn create_code_tree(freq_table: &Vec<u32>) -> Result<Box<Node>, Box<dyn Error>> {
    let mut heap = BinaryHeap::new();

    // add the frequency table nodes to priority queue
    for i in 0..TABLE_SIZE {
        let freq = freq_table[i];
        if freq != 0 {
            heap.push(Box::new(Node::leaf(i as u8, freq)));
        }
    }

    // huffman coding algorithm
    while heap.len() > 1 {
        let first_node = heap.pop().ok_or("First node is None")?;
        let second_node = heap.pop().ok_or("Second node is None")?;
        let w = first_node.weight + second_node.weight;
        heap.push(Box::new(Node::internal(first_node, second_node, 0, w)));
    }

    let node = heap.pop().ok_or("Heap is empty after algorithm")?;
    Ok(node)
}

fn walk_code_tree(node: &Box<Node>, mut symbol_code: bitwise::SymbolCode, symbol_table: &mut Vec<bitwise::SymbolCode>) {
    if node.is_root() {
        symbol_code.plain_symbol = node.plain_symbol;
        symbol_table[usize::from(node.plain_symbol)] = symbol_code;
    }
    if let Some(left) = &node.left {
        let symbol_code = symbol_code.append_bit(0);
        walk_code_tree(left, symbol_code, symbol_table);
    }
    if let Some(right) = &node.right {
        let symbol_code = symbol_code.append_bit(1);
        walk_code_tree(right, symbol_code, symbol_table);
    }
}

fn create_code_table(root: &Box<Node>) -> Result<Vec<bitwise::SymbolCode>, Box<dyn Error>> {
    let symbol_code = bitwise::SymbolCode { plain_symbol: 0, encoded_symbol: 0, bit_len: 0 };
    let mut symbol_table = vec![symbol_code; TABLE_SIZE];
    walk_code_tree(root, symbol_code, &mut symbol_table);
    Ok(symbol_table)
}