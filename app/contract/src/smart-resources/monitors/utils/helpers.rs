use std::time::{SystemTime, UNIX_EPOCH};

// Function to convert a SystemTime to a UNIX timestamp
pub fn system_time_to_unix_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Function to format a percentage value
pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}

// Function to merge two HashMaps
pub fn merge_hashmaps<K, V>(map1: &mut HashMap<K, V>, map2: HashMap<K, V>)
where
    K: std::cmp::Eq + std::hash::Hash,
{
    for (key, value) in map2 {
        map1.insert(key, value);
    }
}

// Example usage
fn main() {
    let now = SystemTime::now();
    println!("Current UNIX timestamp: {}", system_time_to_unix_timestamp(now));

    let percentage = 45.6789;
    println!("Formatted percentage: {}", format_percentage(percentage));

    let mut map1 = HashMap::new();
    map1.insert("key1", "value1");
    let mut map2 = HashMap::new();
    map2.insert("key2", "value2");
    merge_hashmaps(&mut map1, map2);
    println!("Merged HashMap: {:?}", map1);
}