use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    str,
};

// ChunkFileReader reads from file in chunks of up to chunk_size bytes;
pub struct ChunkFileReader {
    f: File,
    chunk_size: u16,
}

impl ChunkFileReader {
    // new is buffered reader constructor
    pub fn new(file_name: &str, chunk_size: u16) -> std::io::Result<ChunkFileReader> {
        let file = File::open(file_name)?;

        Ok(ChunkFileReader {
            f: file,
            chunk_size: chunk_size,
        })
    }

    // read applies provided visitor to each chunk read from the file.
    pub fn read<T: Fn(&str) -> bool>(&self, visitor: &T) -> Result<(), Box<dyn Error>> {
        let mut reader = BufReader::new(&self.f);
        let mut buffer = vec![0u8; usize::from(self.chunk_size)];

        // read file in chunks and pass to the visitor
        // this while loop shall run until
        loop {
            match reader.read(&mut buffer)? {
                // since this loop is also the last expression of the function,
                // we may specify the return value here directly, instead of writing it below
                // the loop body!
                0 => break Ok(()),
                n => {
                    if !visitor(std::str::from_utf8(&buffer[..n])?) {
                        break Ok(());
                    }
                }
            }
        }
    }
}
