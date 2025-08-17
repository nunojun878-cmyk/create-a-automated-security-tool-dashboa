// Import necessary libraries
extern crate reqwest;
extern crate serde_json;

use reqwest::Client;
use serde_json::Value;

// Define a struct to hold the dashboard data
struct SecurityDashboard {
    vulnerabilities: Vec<Vulnerability>,
    system_info: SystemInfo,
}

struct Vulnerability {
    id: i32,
    name: String,
    severity: String,
}

struct SystemInfo {
    os: String,
    kernel: String,
}

// Function to fetch vulnerability data from an API
async fn fetch_vulnerabilities(client: &Client) -> Vec<Vulnerability> {
    let res = client.get("https://vulnerability-api.com/data")
        .send()
        .await
        .unwrap();
    let json: Value = res.json().await.unwrap();
    let mut vulnerabilities = vec![];
    for item in json.as_array().unwrap() {
        let id = item["id"].as_i64().unwrap() as i32;
        let name = item["name"].as_str().unwrap().to_string();
        let severity = item["severity"].as_str().unwrap().to_string();
        vulnerabilities.push(Vulnerability { id, name, severity });
    }
    vulnerabilities
}

// Function to fetch system information
fn fetch_system_info() -> SystemInfo {
    let os = "Linux".to_string(); // Replace with actual OS detection
    let kernel = "5.15.0-53-generic".to_string(); // Replace with actual kernel version
    SystemInfo { os, kernel }
}

// Main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let vulnerabilities = fetch_vulnerabilities(&client).await;
    let system_info = fetch_system_info();
    let dashboard = SecurityDashboard { vulnerabilities, system_info };
    
    // Display the dashboard data
    println!("Security Dashboard");
    println!("----------------");
    println!("System Information:");
    println!("  OS: {}", dashboard.system_info.os);
    println!("  Kernel: {}", dashboard.system_info.kernel);
    println!("Vulnerabilities:");
    for vuln in dashboard.vulnerabilities {
        println!("  {} ({}): {}", vuln.id, vuln.severity, vuln.name);
    }
    
    Ok(())
}