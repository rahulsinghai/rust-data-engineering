/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // for shuffle. rand is a random number generation library in Rust
use rand::rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<&str> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<&str> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");
    
    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Add an element to the middle of the list
    let mut cursor = fruit.cursor_front_mut();
    for _ in 0..(fruit.len() / 2) {
        cursor.move_next();
    }
    cursor.insert_after("Mango");

    // Print out the fruit salad after adding Mango
    println!("Fruit Salad after adding Mango:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Remove the element from the middle of the list
    cursor = fruit.cursor_front_mut();
    for _ in 0..(fruit.len() / 2) {
        cursor.move_next();
    }
    cursor.remove_current();

    // Print out the fruit salad after removing Mango
    println!("Fruit Salad after removing Mango:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
