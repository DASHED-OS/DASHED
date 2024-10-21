mod metrics {
    pub mod collections {
        pub mod cpu;
        pub mod memory;
        pub mod network;
        pub mod storage;
    }
    pub mod models {
        pub mod metric;
        pub mod report;
        pub mod threshold;
    }
    pub mod services {
        pub mod alerting;
        pub mod analysis;
        pub mod reporting;
    }
    pub mod utils {
        pub mod config;
        pub mod helpers;
    }
}

mod imports {
    pub mod services {
        pub mod audit;
        pub mod import_manager;
        pub mod security;
    }
    pub mod formats {
        pub mod csv;
        pub mod json;
    }
    pub mod models {
        pub mod import_log;
        pub mod import_task;
        pub mod import_policy;
    }
    pub mod utils {
        pub mod config;
        pub mod helper;
    }
}

mod exports {
    pub mod services {
        pub mod audit;
        pub mod export_manager;
        pub mod security;
    }
    pub mod formats {
        pub mod csv;
        pub mod json;
    }
    pub mod models {
        pub mod export_log;
        pub mod export_task;
        pub mod export_policy;
    }
    pub mod utils {
        pub mod config;
        pub mod helper;
    }
}

use metrics::collections::cpu::*;
use metrics::collections::memory::*;
use metrics::collections::network::*;
use metrics::collections::storage::*;
use metrics::services::alerting::*;
use metrics::services::analysis::*;
use metrics::services::reporting::*;
use metrics::models::report::*;
use metrics::models::threshold::*;
use metrics::utils::config::*;
use metrics::utils::helpers::*;

fn main() {
    // Collect and report CPU metrics
    let cpu_metrics = collect_cpu_metrics();
    println!("Collected CPU Metrics: {:?}", cpu_metrics);

    // Collect and report memory metrics
    let memory_metrics = collect_memory_metrics();
    println!("Collected Memory Metrics: {:?}", memory_metrics);

    // Collect and report network metrics
    let network_metrics = collect_network_metrics();
    println!("Collected Network Metrics: {:?}", network_metrics);

    // Collect and report storage metrics
    let storage_metrics = collect_storage_metrics();
    println!("Collected Storage Metrics: {:?}", storage_metrics);

    // Generate a report
    let mut metrics = HashMap::new();
    metrics.insert("CPU Usage".to_string(), cpu_metrics.usage_percent as f64);
    metrics.insert("Memory Usage".to_string(), memory_metrics.used_memory_mb as f64);
    let report = generate_metrics_report(&metrics);
    println!("Generated Report: {:?}", report);

    // Analyze CPU metrics
    let analysis_result = analyze_cpu_metrics(cpu_metrics.usage_percent, &cpu_metrics.core_usages);
    println!("Analysis Result: {:?}", analysis_result);

    // Create an alert if necessary
    if cpu_metrics.usage_percent > 80.0 {
        let mut alert = create_alert("High CPU Usage", "CPU usage exceeded 80%");
        add_alert_detail(&mut alert, "current_usage", &cpu_metrics.usage_percent.to_string());
        println!("Alert: {:?}", alert);
    }
}