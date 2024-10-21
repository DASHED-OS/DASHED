use std::collections::HashMap;
use std::time::SystemTime;

// Function to format a timestamp into a readable string
pub fn format_timestamp(timestamp: SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Utc> = timestamp.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Function to merge two HashMaps
pub fn merge_hashmaps<K, V>(map1: HashMap<K, V>, map2: HashMap<K, V>) -> HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    map1.into_iter().chain(map2.into_iter()).collect()
}

// Example usage
fn main() {
    let timestamp = SystemTime::now();
    println!("Formatted Timestamp: {}", format_timestamp(timestamp));

    let mut map1 = HashMap::new();
    map1.insert("key1", "value1");
    let mut map2 = HashMap::new();
    map2.insert("key2", "value2");

    let merged_map = merge_hashmaps(map1, map2);
    println!("Merged HashMap: {:?}", merged_map);
}