use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to import a list of service definitions from a CSV file
pub fn import_services_from_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<ServiceDefinition>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 {
            // Skip the header line
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 6 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid CSV format"));
        }

        let service = ServiceDefinition {
            name: parts[0].to_string(),
            image: parts[1].to_string(),
            ports: parts[2].split(';').map(|s| s.to_string()).collect(),
            environment: parts[3].split(';').map(|s| {
                let kv: Vec<&str> = s.split('=').collect();
                (kv[0].to_string(), kv[1].to_string())
            }).collect(),
            volumes: parts[4].split(';').map(|s| s.to_string()).collect(),
            networks: parts[5].split(';').map(|s| s.to_string()).collect(),
        };

        services.push(service);
    }

    Ok(services)
}

// Example usage
fn main() -> io::Result<()> {
    let services = import_services_from_csv("services.csv")?;
    for service in services {
        println!("Imported Service: {:?}", service);
    }
    Ok(())
}