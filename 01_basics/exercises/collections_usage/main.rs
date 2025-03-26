use std::collections::{HashMap, HashSet};

fn main() {
    println!("Basic Collections Demo");
    println!("--------------------");

    // Vector demonstrations
    vector_operations();

    println!("\n--------------------");

    // HashMap demonstrations
    hashmap_operations();

    println!("\n--------------------");

    // HashSet demonstrations
    hashset_operations();
}

fn vector_operations() {
    println!("Vector Operations:");
    
    // Creating and modifying a vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", numbers);

    // Adding elements
    numbers.push(6);
    numbers.extend(vec![7, 8]);
    println!("After adding elements: {:?}", numbers);

    // Removing elements
    numbers.pop();
    println!("After popping last element: {:?}", numbers);

    // Vector sorting
    numbers.sort();
    println!("Sorted vector: {:?}", numbers);

    // Vector filtering
    let even_numbers: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    println!("Even numbers: {:?}", even_numbers);

    // Vector mapping
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled numbers: {:?}", doubled);
}

fn hashmap_operations() {
    println!("HashMap Operations:");
    
    // Creating a HashMap
    let mut scores = HashMap::new();

    // Adding entries
    scores.insert("Alice", 98);
    scores.insert("Bob", 95);
    scores.insert("Charlie", 85);
    println!("Initial scores: {:?}", scores);

    // Updating a value
    scores.insert("Bob", 97);
    println!("After updating Bob's score: {:?}", scores);

    // Getting values
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    // Entry API usage
    scores.entry("David").or_insert(90);
    println!("After adding David: {:?}", scores);

    // Removing entries
    scores.remove("Charlie");
    println!("After removing Charlie: {:?}", scores);
}

fn hashset_operations() {
    println!("HashSet Operations:");
    
    // Creating HashSets
    let mut set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![4, 5, 6, 7, 8].into_iter().collect();

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);

    // Adding and removing elements
    set1.insert(6);
    println!("After inserting 6: {:?}", set1);
    set1.remove(&1);
    println!("After removing 1: {:?}", set1);

    // Set operations
    let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
    println!("Intersection: {:?}", intersection);

    let union: HashSet<_> = set1.union(&set2).copied().collect();
    println!("Union: {:?}", union);

    let difference: HashSet<_> = set1.difference(&set2).copied().collect();
    println!("Difference (set1 - set2): {:?}", difference);
}