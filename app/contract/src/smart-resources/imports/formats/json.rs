use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use serde_json;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to import a list of service definitions from a JSON file
pub fn import_services_from_json<P: AsRef<Path>>(path: P) -> io::Result<Vec<ServiceDefinition>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let services: Vec<ServiceDefinition> = serde_json::from_reader(reader)?;
    Ok(services)
}

// Example usage
fn main() -> io::Result<()> {
    let services = import_services_from_json("services.json")?;
    for service in services {
        println!("Imported Service: {:?}", service);
    }
    Ok(())
}