pub mod reader;
pub mod primes;

use std::fmt;

// this syntax will derive (implement automatically) debug trait for Person.
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u16,
}

// In order to use non-debug printing (fmt::Display), one has
// to manually implement this fmt::Display trait.
// This looks much like C++ concepts with better compiler support to me.
// I should inspect how traits are implemented in Rust at runtime.
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // you can remove semicolon on the last line to
        // return a value implicitly
        write!(f, "Ayoo! This is {}, {} y.o.", self.name, self.age)
    }
}

pub fn inspect_strings() {
    use std::mem;

    let story = String::from("Once upon a time...");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    // assert a false thing for a reason
    assert_ne!(ptr, std::ptr::null_mut::<u8>());
    assert_eq!(len, 19);
    assert_eq!(capacity, len);

    // drop the String's data (can only be done in an unsafe block)
    // so that all memory is deallocated
    unsafe {
        mem::ManuallyDrop::drop(&mut story);
    }
}
