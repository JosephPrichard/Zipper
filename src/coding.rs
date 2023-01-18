// Joseph Prichard
// 1/5/2023
// Huffman coding implementation using bin trees and symbol tables

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::{bitwise};
use crate::files::BitFileStream;

pub const TABLE_SIZE: usize = 256;

pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub symbol: u8,
    pub weight: u32
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub fn create_freq_table(input_filepath: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    // keep track of symbol counts
    let mut freq_table = vec![0u32; TABLE_SIZE];

    // iterate through each byte in the file
    let mut fbs = BitFileStream::new(input_filepath)?;
    while let Some(byte) = fbs.consume_byte()? {
        freq_table[usize::from(byte)] += 1;
    }

    Ok(freq_table)
}

pub fn create_code_tree(freq_table: &Vec<u32>) -> Result<Box<Node>, Box<dyn Error>> {
    let mut heap = BinaryHeap::new();

    // add the frequency table nodes to priority queue
    for i in 0..TABLE_SIZE {
        let freq = freq_table[i];
        if freq != 0 {
            let node = Box::new(Node {
                left: None,
                right: None,
                symbol: i as u8,
                weight: freq
            });
            heap.push(node);
        }
    }

    // huffman coding algorithm
    while heap.len() > 1 {
        let first_node = heap.pop().ok_or("First node is None")?;
        let second_node = heap.pop().ok_or("Second node is None")?;
        let w = first_node.weight + second_node.weight;
        let node = Box::new(Node {
            left: Some(Box::new(*first_node)),
            right: Some(Box::new(*second_node)),
            symbol: 0,
            weight: w
        });
        heap.push(node);
    }

    let node = heap.pop().ok_or("Heap is empty after algorithm")?;
    Ok(node)
}

pub fn walk_code_tree(node: Box<Node>, mut symbol_code: bitwise::SymbolCode, symbol_table: &mut Vec<bitwise::SymbolCode>) {
    // root symbol is added to symbol table
    if node.left == None && node.right == None {
        symbol_code.symbol = node.symbol;
        symbol_table[usize::from(node.symbol)] = symbol_code;
    }
    if let Some(left) = node.left {
        let symbol_code = symbol_code.append_bit(0);
        walk_code_tree(left, symbol_code, symbol_table);
    }
    if let Some(right) = node.right {
        let symbol_code = symbol_code.append_bit(1);
        walk_code_tree(right, symbol_code, symbol_table);
    }
}

pub fn create_code_table(root: Box<Node>) -> Result<Vec<bitwise::SymbolCode>, Box<dyn Error>> {
    let symbol_code = bitwise::SymbolCode { symbol: 0, code: 0, bit_len: 0 };
    let mut symbol_table = vec![symbol_code; TABLE_SIZE];
    walk_code_tree(root, symbol_code, &mut symbol_table);
    Ok(symbol_table)
}