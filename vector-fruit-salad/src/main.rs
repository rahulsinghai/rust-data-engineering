/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

// rand is a random number generation library in Rust
use rand::seq::SliceRandom; // imports the SliceRandom trait from the rand crate, which provides the shuffle method for slices
use rand::rng; // imports a random number generator

fn main() {
    // A mutable vector of strings is created, containing the names of various fruits:
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Scramble (shuffle) the fruit
    let mut rng = rng(); // create a random number generator.
    fruit.shuffle(&mut rng);  // (&mut rng): This is a mutable reference to the random number generator. The rng is created earlier in the code using let mut rng = thread_rng();. The shuffle method requires a mutable reference to a random number generator to perform the shuffling.

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
