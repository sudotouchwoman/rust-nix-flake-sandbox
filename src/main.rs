use std::{env, process};

use sandbox::samples;

const EXIT_FAILURE: i32 = 1;

fn main() {
    let foo = "suck on this";

    println!("{}", sandbox::public_hello(foo));

    let args: Vec<String> = env::args().collect();

    // expect single argument
    if args.len() != 2 {
        eprintln!("Please provide a file name");
        process::exit(EXIT_FAILURE);
    }

    // get a reference to the second argument (filename)
    let filename = &args[1];

    const CHUNK_SIZE: u16 = 0x100;

    let reader = samples::reader::ChunkFileReader::new(&filename, CHUNK_SIZE).unwrap_or_else(|e| {
        eprintln!("Failed to open file: {}: {}", filename, e);
        process::exit(EXIT_FAILURE);
    });

    reader
        .read(&|chunk| {
            println!("[{len}]: '{chunk}'", len = chunk.len(), chunk = chunk);
            true
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to read line: {}: {}", filename, e);
            process::exit(1);
        });

    println!("Done, have a nice night!");
}
