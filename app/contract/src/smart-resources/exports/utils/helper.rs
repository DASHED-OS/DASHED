use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Function to convert a SystemTime to a UNIX timestamp
pub fn system_time_to_unix_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Function to merge two HashMaps
pub fn merge_hashmaps<K, V>(map1: &HashMap<K, V>, map2: &HashMap<K, V>) -> HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut merged = map1.clone();
    for (key, value) in map2 {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

// Example usage
fn main() {
    let time = SystemTime::now();
    let timestamp = system_time_to_unix_timestamp(time);
    println!("Current UNIX timestamp: {}", timestamp);

    let map1: HashMap<String, String> = [("key1".to_string(), "value1".to_string())].iter().cloned().collect();
    let map2: HashMap<String, String> = [("key2".to_string(), "value2".to_string())].iter().cloned().collect();
    let merged_map = merge_hashmaps(&map1, &map2);
    println!("Merged HashMap: {:?}", merged_map);
}