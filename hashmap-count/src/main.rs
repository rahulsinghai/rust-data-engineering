/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for num in numbers {
        // entry(num) returns an Entry enum that represents the key-value pair in the HashMap.
        // or_insert(0) inserts the key `num` with a value of 0 if the key does not exist and returns a mutable reference to the value corresponding to `num`.
        // The value represents the frequency count of the number `num`.
        let frequency = frequencies.entry(num).or_insert(0);

        // The mutable reference is then dereferenced and incremented by 1.
        *frequency += 1;
    }

    let mut result: Vec<(i32, u32)> = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    // :? is used to print the result in a debug format.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
