use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    // `into_iter` takes ownership of `fruits` vector and returns an iterator
    // `fruits` can no longer be used here because it has been consumed
    // `take` takes the first `num_fruits` elements from the iterator
    // `collect` collects the elements into a new vector, which is returned from the function
    fruits.into_iter().take(num_fruits).collect()
}
