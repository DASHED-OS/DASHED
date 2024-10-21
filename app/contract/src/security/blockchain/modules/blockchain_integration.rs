use std::collections::HashMap;

// Define a struct to represent a blockchain integration
#[derive(Debug, Clone)]
pub struct BlockchainIntegration {
    pub chain_name: String,
    pub nodes: Vec<String>,
}

// Function to add a new blockchain integration
pub fn add_blockchain_integration(
    integrations: &mut HashMap<String, BlockchainIntegration>, 
    chain_name: &str, 
    nodes: Vec<String>
) {
    let integration = BlockchainIntegration {
        chain_name: chain_name.to_string(),
        nodes,
    };
    integrations.insert(chain_name.to_string(), integration);
}

// Function to get nodes of a specific blockchain integration
pub fn get_blockchain_nodes(
    integrations: &HashMap<String, BlockchainIntegration>, 
    chain_name: &str
) -> Option<Vec<String>> {
    integrations.get(chain_name).map(|integration| integration.nodes.clone())
}

// Function to list all blockchain integrations
pub fn list_blockchain_integrations(integrations: &HashMap<String, BlockchainIntegration>) -> Vec<String> {
    integrations.keys().cloned().collect()
}

// Example usage
fn main() {
    let mut integrations = HashMap::new();

    add_blockchain_integration(&mut integrations, "Ethereum", vec!["Node1".to_string(), "Node2".to_string()]);
    add_blockchain_integration(&mut integrations, "Bitcoin", vec!["Node3".to_string(), "Node4".to_string()]);

    if let Some(nodes) = get_blockchain_nodes(&integrations, "Ethereum") {
        println!("Ethereum nodes: {:?}", nodes);
    }

    let chain_list = list_blockchain_integrations(&integrations);
    println!("Supported Blockchains: {:?}", chain_list);
}