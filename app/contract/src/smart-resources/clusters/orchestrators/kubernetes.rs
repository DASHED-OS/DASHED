use std::collections::HashMap;

// Define a struct to represent a Kubernetes deployment
#[derive(Debug, Clone)]
pub struct KubernetesDeployment {
    pub deployment_id: String,
    pub deployment_name: String,
    pub replicas: u32,
    pub labels: HashMap<String, String>,
}

// Function to create a new Kubernetes deployment
pub fn create_kubernetes_deployment(deployment_id: &str, deployment_name: &str, replicas: u32) -> KubernetesDeployment {
    KubernetesDeployment {
        deployment_id: deployment_id.to_string(),
        deployment_name: deployment_name.to_string(),
        replicas,
        labels: HashMap::new(),
    }
}

// Function to scale a Kubernetes deployment
pub fn scale_kubernetes_deployment(deployment: &mut KubernetesDeployment, new_replicas: u32) {
    println!("Scaling Kubernetes deployment: {:?}", deployment);
    deployment.replicas = new_replicas;
    // Logic to scale the deployment
}

// Function to add a label to a Kubernetes deployment
pub fn add_kubernetes_deployment_label(deployment: &mut KubernetesDeployment, key: &str, value: &str) {
    deployment.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut deployment = create_kubernetes_deployment("kd-1234567890abcdef0", "api-server", 3);
    println!("Created Kubernetes Deployment: {:?}", deployment);

    scale_kubernetes_deployment(&mut deployment, 5);
    println!("Scaled Kubernetes Deployment: {:?}", deployment);

    add_kubernetes_deployment_label(&mut deployment, "version", "v1.0");
    println!("Updated Kubernetes Deployment Labels: {:?}", deployment);
}