use crate::files::BitFileStream;

mod compress;
mod files;
mod decompress;
mod bitwise;
mod coding;

fn main() {
    compress::compress_file(
        "C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test.txt",
        "C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test2.txt"
    );

    // test file bit stream
    let mut f = BitFileStream::new("C:\\Users\\Joseph\\OneDrive\\Documents\\Programs\\Rust\\Zip\\input\\test.txt")
        .expect("err");
    while let Some(bit) = f.consume_bit().expect("") {
        print!("{}", bit);
    }
    println!("\nFinished");
}
