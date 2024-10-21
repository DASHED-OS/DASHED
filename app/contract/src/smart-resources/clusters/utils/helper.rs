use std::collections::HashMap;

// Function to merge two HashMaps
pub fn merge_hashmaps<K: Eq + std::hash::Hash, V>(map1: &HashMap<K, V>, map2: &HashMap<K, V>) -> HashMap<K, V>
where
    V: Clone,
{
    let mut merged = map1.clone();
    for (key, value) in map2 {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

// Function to print a formatted message
pub fn print_formatted_message(message: &str) {
    println!("--- {} ---", message);
}

// Example usage
fn main() {
    let mut map1 = HashMap::new();
    map1.insert("key1", "value1");
    map1.insert("key2", "value2");

    let mut map2 = HashMap::new();
    map2.insert("key3", "value3");
    map2.insert("key4", "value4");

    let merged_map = merge_hashmaps(&map1, &map2);
    println!("Merged HashMap: {:?}", merged_map);

    print_formatted_message("Operation Completed");
}