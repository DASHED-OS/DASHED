use std::collections::HashMap;

// Define a struct to represent a Nomad job
#[derive(Debug, Clone)]
pub struct NomadJob {
    pub job_id: String,
    pub job_name: String,
    pub task_count: u32,
    pub labels: HashMap<String, String>,
}

// Function to create a new Nomad job
pub fn create_nomad_job(job_id: &str, job_name: &str, task_count: u32) -> NomadJob {
    NomadJob {
        job_id: job_id.to_string(),
        job_name: job_name.to_string(),
        task_count,
        labels: HashMap::new(),
    }
}

// Function to scale a Nomad job
pub fn scale_nomad_job(job: &mut NomadJob, new_task_count: u32) {
    println!("Scaling Nomad job: {:?}", job);
    job.task_count = new_task_count;
    // Logic to scale the job
}

// Function to add a label to a Nomad job
pub fn add_nomad_job_label(job: &mut NomadJob, key: &str, value: &str) {
    job.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut job = create_nomad_job("nj-1234567890abcdef0", "data-processing", 10);
    println!("Created Nomad Job: {:?}", job);

    scale_nomad_job(&mut job, 15);
    println!("Scaled Nomad Job: {:?}", job);

    add_nomad_job_label(&mut job, "priority", "high");
    println!("Updated Nomad Job Labels: {:?}", job);
}