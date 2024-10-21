use crate::smart_resources::clouds::models::cloud_instance::CloudInstance;
use crate::smart_resources::clouds::models::storage::StorageConfig;
use crate::smart_resources::clouds::models::network::NetworkConfig;
use crate::smart_resources::clouds::providers::aws::AwsInstance;
use crate::smart_resources::clouds::providers::azure::AzureInstance;
use crate::smart_resources::clouds::providers::gcp::GcpInstance;

// Function to scale a generic cloud instance
pub fn scale_instance(instance: &mut CloudInstance, new_instance_type: &str) {
    println!("Scaling cloud instance: {:?}", instance);
    instance.instance_type = new_instance_type.to_string();
    // Logic to scale the instance
}

// Function to scale an AWS instance
pub fn scale_aws_instance(instance: &mut AwsInstance, new_instance_type: &str) {
    println!("Scaling AWS instance: {:?}", instance);
    instance.instance_type = new_instance_type.to_string();
    // Logic to scale the AWS instance
}

// Function to scale an Azure instance
pub fn scale_azure_instance(instance: &mut AzureInstance, new_instance_type: &str) {
    println!("Scaling Azure instance: {:?}", instance);
    instance.instance_type = new_instance_type.to_string();
    // Logic to scale the Azure instance
}

// Function to scale a GCP instance
pub fn scale_gcp_instance(instance: &mut GcpInstance, new_instance_type: &str) {
    println!("Scaling GCP instance: {:?}", instance);
    instance.instance_type = new_instance_type.to_string();
    // Logic to scale the GCP instance
}

// Example usage
fn main() {
    let mut cloud_instance = CloudInstance {
        instance_id: "i-1234567890abcdef0".to_string(),
        instance_type: "t2.micro".to_string(),
        status: "running".to_string(),
        metadata: std::collections::HashMap::new(),
    };
    scale_instance(&mut cloud_instance, "t2.large");

    let mut aws_instance = AwsInstance {
        instance_id: "i-1234567890abcdef0".to_string(),
        instance_type: "t2.micro".to_string(),
        region: "us-west-2".to_string(),
        status: "running".to_string(),
        tags: std::collections::HashMap::new(),
    };
    scale_aws_instance(&mut aws_instance, "t2.large");

    let mut azure_instance = AzureInstance {
        instance_id: "az-1234567890abcdef0".to_string(),
        instance_type: "Standard_B1s".to_string(),
        region: "eastus".to_string(),
        status: "running".to_string(),
        tags: std::collections::HashMap::new(),
    };
    scale_azure_instance(&mut azure_instance, "Standard_B2s");

    let mut gcp_instance = GcpInstance {
        instance_id: "gcp-1234567890abcdef0".to_string(),
        instance_type: "n1-standard-1".to_string(),
        zone: "us-central1-a".to_string(),
        status: "running".to_string(),
        labels: std::collections::HashMap::new(),
    };
    scale_gcp_instance(&mut gcp_instance, "n1-standard-2");
}