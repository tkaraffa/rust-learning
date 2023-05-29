use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

const API_BASE_URL: &str = "https://api.bls.gov/publicAPI/v2/";

#[derive(Debug, Deserialize, Serialize)]
struct BLSRequest {
    seriesid: Vec<String>,
    startyear: String,
    endyear: String,
    catalog: bool,
    calculations: bool,
    annualaverage: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct BLSResponse {
    status: String,
    response_time: u64,
    message: Vec<String>,
    results: Option<Vec<BLSResult>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BLSResult {
    seriesID: String,
    data: Vec<BLSData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BLSData {
    year: String,
    period: String,
    value: String,
    footnote_codes: Option<Vec<String>>,
}

fn main() {
    // Get the API key and series ID from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <API key> <series ID>", args[0]);
        return;
    }
    let api_key = &args[1];
    let series_id = &args[2];

    // Set up the BLS API request
    let request = BLSRequest {
        seriesid: vec![series_id.to_string()],
        startyear: "2010".to_string(),
        endyear: "2020".to_string(),
        catalog: false,
        calculations: false,
        annualaverage: false,
    };

    // Send the request to the BLS API
    let client = reqwest::Client::new();
    let response = client
        .post(API_BASE_URL)
        .header("Content-Type", "application/json")
        .header("BLS-API-Key", api_key)
        .json(&request)
        .send()
        .unwrap()
        .json::<BLSResponse>()
        .unwrap();

    // Check if the request was successful and print the data if it was
    if response.status == "REQUEST_SUCCEEDED" {
        let result = &response.results.unwrap()[0];
        println!("Series ID: {}", result.seriesID);
        for data in &result.data {
            println!("{} {}: {}", data.year, data.period, data.value);
        }
    } else {
        println!("Error: {}", response.message.join(", "));
    }
}
