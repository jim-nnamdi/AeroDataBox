use std::fs::rename;

// use reqwest::blocking::{Error};
use chrono::{DateTime, Local};

use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct Aircraft{
    model: String,
    reg: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct Airline{
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Location{
    lat : f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Airport{
    countryCode:String,
    iata:String,
    icao:String,
    location:Location,
    municipalityName:String,
    name:String,
    shortName:String
}

#[derive(Debug, Serialize, Deserialize)]
struct Quality {
    #[serde = rename(0)]
    basic: String,

    #[serde=rename(1)]
    live: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Arrival {
    #[serde(rename="actualTimeLocal")]
    actual_time_local: Option<DateTime<Local>>,

    #[serde(rename="actualTimeUTC")]
    actual_time_utc : Option<DateTime<Local>>,

    airport: Airport,
    baggageBelt: String,
    gate: String,
    quality:Vec<Quality>,
    scheduledTimeLocal: Option<DateTime<Local>>,
    scheduledTimeUTC: Option<DateTime<Local>>,
    terminal: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct FlightData{
    aircraft: Aircraft,
    airline: Airline,
    arrival: Arrival,
    
    #[serde(rename="camelcase")]
    callsign: String,

    #[serde(rename="codeshareStatus")]
    code_share_status: String,
    isCargo: bool,
    status: String,
}

#[tokio::main]
pub async fn flight() -> Result<(), reqwest::Error>{

    const API_KEY: &str = "53fd0041f2msh8c3ffa5b5508be0p152202jsn9a0f742df4a8";
    const API_HOST : &str = "aerodatabox.p.rapidapi.com";

    let aerobox_client = reqwest::Client::new();

    let flight_status_request = aerobox_client.get("https://aerodatabox.p.rapidapi.com/flights/number/KL1395/2022-09-30")
    .header("X-RapidAPI-Key", API_KEY)
    .header("X-RapidAPI-Host",API_HOST )
    .send()
    .await?
    .text()
    .await?;

    println!("{:#?}", flight_status_request);
    Ok(())
}