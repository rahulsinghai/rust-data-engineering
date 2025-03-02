// This program demonstrates the use of a `BTreeSet` to store a unique set of fruits.
// It randomly shuffles a list of fruits and inserts a specified number of them into the set.
// The number of fruits to insert is determined by the `amounts` array.

use rand::seq::SliceRandom;
use rand::rng;
use std::collections::BTreeSet;

fn main() {
    // Defines a list of fruits and an array of amounts.
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    // The number of fruits to insert into the set for each amount.
    let amounts = [1, 3, 5, 7, 9];

    // Initializes the random number generator
    let mut rng = rng();

    // Iterates over each amount in the `amounts` array. For each amount, it:
    //    a. Creates a new `BTreeSet` to store the fruits.
    //    b. Clones and shuffles the list of fruits.
    //    c. Inserts fruits into the set until the set contains the specified number of fruits.
    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        // Prints the amount and the corresponding set of fruits.
        // :? is a format specifier that prints the set of fruits.
        println!("{}: {:?}", amount, fruit_set);
    }
}
