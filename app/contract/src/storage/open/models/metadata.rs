use std::collections::HashMap;

// Define a struct to represent metadata for a storage item
#[derive(Debug, Clone)]
pub struct Metadata {
    pub item_id: String,
    pub attributes: HashMap<String, String>,
}

// Function to create a new metadata entry
pub fn create_metadata(item_id: &str) -> Metadata {
    Metadata {
        item_id: item_id.to_string(),
        attributes: HashMap::new(),
    }
}

// Function to add an attribute to metadata
pub fn add_metadata_attribute(metadata: &mut Metadata, key: &str, value: &str) {
    metadata.attributes.insert(key.to_string(), value.to_string());
}

// Function to get an attribute from metadata
pub fn get_metadata_attribute(metadata: &Metadata, key: &str) -> Option<String> {
    metadata.attributes.get(key).cloned()
}

// Example usage
fn main() {
    let mut metadata = create_metadata("item-12345678");
    println!("Created Metadata: {:?}", metadata);

    add_metadata_attribute(&mut metadata, "author", "user123");
    println!("Updated Metadata: {:?}", metadata);

    if let Some(author) = get_metadata_attribute(&metadata, "author") {
        println!("Author: {}", author);
    }
}