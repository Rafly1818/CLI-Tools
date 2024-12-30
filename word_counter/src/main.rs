use std::env;
use std::fs;

fn main() {
    // Ambil argumen dari command-line
    let args: Vec<String> = env::args().collect();
    
    // Cek apakah ada file yang diinput
    if args.len() < 2 {
        eprintln!("Usage: word_counter <filename>");
        return;
    }

    let filename = &args[1];

    // Baca file yang diinput
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Hitung jumlah kata
    let word_count = contents.split_whitespace().count();

    println!("File '{}' memiliki {} kata.", filename, word_count);
}
