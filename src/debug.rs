use crate::read::FileReader;

pub fn dump_binary_file(filepath: &str) {
    let mut reader = FileReader::new(filepath).expect("Error creating file reader");
    println!();
    let mut c = 0;
    while let Some(bit) = reader.read_bit().expect("Error reading bit from file") {
        print!("{}", bit);
        if (c + 1) % 4 == 0 {
            print!(" ");
        }
        c += 1;
    }
}

