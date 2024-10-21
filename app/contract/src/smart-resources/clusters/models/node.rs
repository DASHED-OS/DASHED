use std::collections::HashMap;

// Define a struct to represent a node in a cluster
#[derive(Debug, Clone)]
pub struct Node {
    pub node_id: String,
    pub node_type: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

// Function to create a new node
pub fn create_node(node_id: &str, node_type: &str) -> Node {
    Node {
        node_id: node_id.to_string(),
        node_type: node_type.to_string(),
        status: "active".to_string(),
        labels: HashMap::new(),
    }
}

// Function to update the status of a node
pub fn update_node_status(node: &mut Node, new_status: &str) {
    node.status = new_status.to_string();
}

// Function to add a label to a node
pub fn add_node_label(node: &mut Node, key: &str, value: &str) {
    node.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut node = create_node("n-1234567890abcdef0", "compute");
    println!("Created Node: {:?}", node);

    update_node_status(&mut node, "inactive");
    println!("Updated Node Status: {:?}", node);

    add_node_label(&mut node, "role", "worker");
    println!("Updated Node Labels: {:?}", node);
}