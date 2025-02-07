fn main() {
    let foo = "suck on this";
    println!("Hello, world! {}", foo);

    let me = Person {
        name: "sudotouchwoman".to_string(),
        age: 23,
    };

    println!(
        "Hello, {name}! your age is: {age}",
        name = me.name,
        age = me.age
    );

    if me.age > 20 {
        println!("Looks like your age is greater than 20!");
    }

    inspect_strings();
}

struct Person {
    name: String,
    age: u16,
}

fn inspect_strings() {
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
