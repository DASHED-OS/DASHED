use std::collections::HashMap;
use crate::smart_resources::clouds::models::cloud_instance::CloudInstance;
use crate::smart_resources::clouds::models::storage::StorageConfig;
use crate::smart_resources::clouds::models::network::NetworkConfig;
use crate::smart_resources::clouds::providers::aws::AwsInstance;
use crate::smart_resources::clouds::providers::azure::AzureInstance;
use crate::smart_resources::clouds::providers::gcp::GcpInstance;

// Function to print details of a cloud instance
pub fn print_instance_details(instance: &CloudInstance) {
    println!("Instance ID: {}", instance.instance_id);
    println!("Instance Type: {}", instance.instance_type);
    println!("Status: {}", instance.status);
    println!("Metadata: {:?}", instance.metadata);
}

// Function to print details of an AWS instance
pub fn print_aws_instance_details(instance: &AwsInstance) {
    println!("AWS Instance ID: {}", instance.instance_id);
    println!("Instance Type: {}", instance.instance_type);
    println!("Region: {}", instance.region);
    println!("Status: {}", instance.status);
    println!("Tags: {:?}", instance.tags);
}

// Function to print details of an Azure instance
pub fn print_azure_instance_details(instance: &AzureInstance) {
    println!("Azure Instance ID: {}", instance.instance_id);
    println!("Instance Type: {}", instance.instance_type);
    println!("Region: {}", instance.region);
    println!("Status: {}", instance.status);
    println!("Tags: {:?}", instance.tags);
}

// Function to print details of a GCP instance
pub fn print_gcp_instance_details(instance: &GcpInstance) {
    println!("GCP Instance ID: {}", instance.instance_id);
    println!("Instance Type: {}", instance.instance_type);
    println!("Zone: {}", instance.zone);
    println!("Status: {}", instance.status);
    println!("Labels: {:?}", instance.labels);
}

// Function to print details of a storage configuration
pub fn print_storage_details(storage: &StorageConfig) {
    println!("Storage ID: {}", storage.storage_id);
    println!("Storage Type: {}", storage.storage_type);
    println!("Capacity (GB): {}", storage.capacity_gb);
    println!("Status: {}", storage.status);
    println!("Tags: {:?}", storage.tags);
}

// Function to print details of a network configuration
pub fn print_network_details(network: &NetworkConfig) {
    println!("Network ID: {}", network.network_id);
    println!("CIDR Block: {}", network.cidr_block);
    println!("Status: {}", network.status);
    println!("Tags: {:?}", network.tags);
}