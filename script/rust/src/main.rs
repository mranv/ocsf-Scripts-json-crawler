use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use serde::Deserialize;

#[derive(Deserialize)]
struct YourDataType {
    // Define the structure that matches the JSON data fields
    // Adjust this based on the actual structure of the JSON data
}

fn get_and_save_json(url: &str, file: &mut std::fs::File) -> Result<(), reqwest::Error> {
    // Fetch JSON data from the URL
    let response = reqwest::blocking::get(url)?;

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Deserialize JSON data
        let json_data: YourDataType = response.json()?;
        
        // Serialize JSON data to a string
        let json_string = serde_json::to_string_pretty(&json_data)?;

        // Save JSON data to the file
        writeln!(file, "{}", json_string)?;

        println!("JSON data saved to file");
        Ok(())
    } else {
        println!("Failed to fetch JSON data. Status code: {}", response.status());
        Err(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch JSON data"))
    }
}

fn main() -> io::Result<()> {
    // Replace the URL with your actual URL
    let url = "http://192.168.43.3:8080/sample/classes/account_change?profiles=";

    // Set the duration for the script to run (5 minutes)
    let duration = Duration::from_secs(5 * 60);

    // Generate a timestamp for the filename
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("all_json_data_{}.json", timestamp);

    // Create or open the file for writing
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    // Record the start time
    let start_time = Instant::now();

    // Run the loop for 5 minutes, refreshing the URL every second
    while Instant::now() - start_time < duration {
        if let Err(err) = get_and_save_json(url, &mut file) {
            eprintln!("Error: {:?}", err);
        }
        std::thread::sleep(Duration::from_secs(1)); // Wait for 1 second before the next iteration
    }

    println!("Script completed.");

    Ok(())
}
