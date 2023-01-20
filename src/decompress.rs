// Joseph Prichard
// 1/5/2023
// Bit-by-bit file decompressor

use std::error::Error;
use crate::read::FileReader;
use crate::tree::Node;
use crate::write::FileWriter;

pub fn decompress_file(input_filepath: &str, output_filepath: &str) {
    let mut reader = &mut FileReader::new(input_filepath).expect("Failed to open file reader");
    let root = read_node(reader).expect("Failed to decode tree from file");
    read_compressed(input_filepath, output_filepath, &root).expect("Failed to decompress file");
}

pub fn read_node(reader: &mut FileReader) -> Result<Box<Node>, Box<dyn Error>> {
    let bit = reader.force_read_bit()?;
    return if bit == 1 {
        Ok(Box::new(Node::leaf(reader.force_read_byte()?, 0)))
    } else {
        let left = read_node(reader)?;
        let right = read_node(reader)?;
        Ok(Box::new(Node::internal(left, right, 0, 0)))
    }
}

pub fn read_compressed(input_filepath: &str, output_filepath: &str, root: &Box<Node>) -> Result<(), Box<dyn Error>> {
    let mut writer = FileWriter::new(output_filepath)?;
    let mut reader = FileReader::new(input_filepath)?;

    let mut node = root;
    while let Some(bit) = reader.read_bit()? {
        if node.is_root() {
            writer.write_byte(node.plain_symbol)?;
            node = &root;
        }
        if bit == 0 {
            if let Some(left) = &root.left {
                node = left;
            }
        } else {
            if let Some(right) = &root.right {
                node = right;
            }
        }
    }
    writer.persist_buffer()?;

    Ok(())
}