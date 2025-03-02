use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

// Generate a random fruit from a list of fruits and return it as a string slice
// String slice is used to avoid allocating a new String for each fruit
// The 'static lifetime is used to indicate that the string slice will live for the entire duration of the program
// This is safe because the fruits are hardcoded and will not be deallocated during the program's execution
fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    // Choose a random fruit from the list. choose() returns an Option<&T>
    // unwrap() is used here because the list of fruits is non-empty
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}
