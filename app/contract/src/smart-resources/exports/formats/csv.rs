use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to export a list of service definitions to a CSV file
pub fn export_services_to_csv<P: AsRef<Path>>(path: P, services: &[ServiceDefinition]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "Name,Image,Ports,Environment,Volumes,Networks")?;

    for service in services {
        let ports = service.ports.join(";");
        let environment: Vec<String> = service.environment.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let environment = environment.join(";");
        let volumes = service.volumes.join(";");
        let networks = service.networks.join(";");

        writeln!(
            file,
            "{},{},{},{},{},{}",
            service.name, service.image, ports, environment, volumes, networks
        )?;
    }

    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    let services = vec![
        ServiceDefinition {
            name: "web".to_string(),
            image: "nginx:latest".to_string(),
            ports: vec!["80:80".to_string()],
            environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
            volumes: vec!["/data".to_string()],
            networks: vec!["frontend".to_string()],
        },
    ];

    export_services_to_csv("services.csv", &services)?;
    println!("Services exported to CSV successfully.");

    Ok(())
}