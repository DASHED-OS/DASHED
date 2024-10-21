use std::collections::HashMap;

// Define a struct to represent an AWS instance
#[derive(Debug, Clone)]
pub struct AwsInstance {
    pub instance_id: String,
    pub instance_type: String,
    pub region: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

// Function to create a new AWS instance
pub fn create_aws_instance(instance_id: &str, instance_type: &str, region: &str) -> AwsInstance {
    AwsInstance {
        instance_id: instance_id.to_string(),
        instance_type: instance_type.to_string(),
        region: region.to_string(),
        status: "running".to_string(),
        tags: HashMap::new(),
    }
}

// Function to update the status of an AWS instance
pub fn update_aws_instance_status(instance: &mut AwsInstance, new_status: &str) {
    instance.status = new_status.to_string();
}

// Function to add a tag to an AWS instance
pub fn add_aws_instance_tag(instance: &mut AwsInstance, key: &str, value: &str) {
    instance.tags.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut instance = create_aws_instance("i-1234567890abcdef0", "t2.micro", "us-west-2");
    println!("Created AWS Instance: {:?}", instance);

    update_aws_instance_status(&mut instance, "stopped");
    println!("Updated AWS Instance Status: {:?}", instance);

    add_aws_instance_tag(&mut instance, "environment", "production");
    println!("Updated AWS Instance Tags: {:?}", instance);
}