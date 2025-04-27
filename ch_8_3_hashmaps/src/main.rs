// HashMap is not part of the prelude so one has to include it from the library
use std::collections::HashMap;

fn main() {
    // Stores the mapping of Key to Value pair using a hashing function 
    // Hashing function determines how it places the key-value pair in the memory

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);

    // Accessing the values in the hash map
    // This is done with the get method -> that returns Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score of the blue team is {score}");

    // Iterating over Key-Value par
    for (key,value) in &scores{
        println!("{key}: {value}")
    }

    // Adding a key value pair only if the key is not existing
    // With entry and or_insert function, key will be added only if it is not existing
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(100);
    let blue_team_score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    let yellow_team_score = scores.get(&String::from("Yellow")).copied().unwrap_or(0);
    println!("It is expected the score of blue is not 50 -> {blue_team_score}");
    println!("It is expected the score of yellow team is 100 -> {yellow_team_score}");
}
