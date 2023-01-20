use std::fs;
use std::path::Path;
use crate::read::FileReader;
use crate::write::FileWriter;

mod compress;
mod read;
mod decompress;
mod bitwise;
mod write;
mod tree;
mod debug;

fn main() {
    let a = "C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test.txt";
    let b = "C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test2.txt";
    let c = "C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test3.txt";

    if Path::new(b).exists() {
        fs::remove_file(b).expect("Error deleting file");
    }
    if Path::new(c).exists() {
        fs::remove_file(c).expect("Error deleting file");
    }

    compress::compress_file(a, b);
    decompress::decompress_file(b, c);

    println!("\nFinished");
}
