use std::{
    fs::File,
    io::{BufReader, Read},
    str,
};

// ChunkFileReader reads from file in chunks of up to chunk_size bytes;
pub struct ChunkFileReader {
    f: File,
    chunk_size: u16,
}

// TODO (sudotouchwoman): implement error enum with thiserror
#[derive(Debug)]
pub enum ReaderError {
    IoError(std::io::Error),
    DecodeError(std::str::Utf8Error),
}

impl From<std::io::Error> for ReaderError {
    fn from(err: std::io::Error) -> ReaderError {
        ReaderError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for ReaderError {
    fn from(err: std::str::Utf8Error) -> ReaderError {
        ReaderError::DecodeError(err)
    }
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
    pub fn read<T: Fn(&str) -> bool>(&self, visitor: &T) -> Result<(), ReaderError> {
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
