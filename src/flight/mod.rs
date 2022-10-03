// use reqwest::blocking::{Error};
use chrono::{DateTime, Local};

use reqwest::{self};
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Aircraft{
    model: String,
    reg: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Airline{
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location{
    lat : f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airport{
    #[serde(rename="countryCode")]
    country_code:String,

    iata:String,
    icao:String,
    location:Location,

    #[serde(rename="municipalityName")]
    municipality_name:String,

    name:String,

    #[serde(rename="shortName")]
    short_name:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    basic: String,
    live: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrival {
    #[serde(rename="actualTimeLocal")]
    actual_time_local: Option<DateTime<Local>>,

    #[serde(rename="actualTimeUTC")]
    actual_time_utc : Option<DateTime<Local>>,

    airport: Airport,

    #[serde(rename="baggageBelt")]
    baggage_belt: String,
    gate: String,
    quality:Vec<Quality>,

    #[serde(rename="scheduledTimeLocal")]
    scheduled_time_local: Option<DateTime<Local>>,

    #[serde(rename="scheduledTimeUTC")]
    scheduled_time_utc: Option<DateTime<Local>>,
    terminal: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct FlightData{
    aircraft: Aircraft,
    airline: Airline,
    arrival: Arrival,

    #[serde(rename="camelcase")]
    callsign: String,

    #[serde(rename="codeshareStatus")]
    code_share_status: String,

    #[serde(rename="isCargo")]
    is_cargo: bool,
    status: String,
}

#[tokio::main]
pub async fn flight() -> Result<(), reqwest::Error>{

    const API_KEY: &str = "53fd0041f2msh8c3ffa5b5508be0p152202jsn9a0f742df4a8";
    const API_HOST : &str = "aerodatabox.p.rapidapi.com";

    let aerobox_client = reqwest::Client::new();

    let flight_status_request: Vec<FlightData> = aerobox_client.get("https://aerodatabox.p.rapidapi.com/flights/number/KL1395/2022-09-30")
    .header("X-RapidAPI-Key", API_KEY)
    .header("X-RapidAPI-Host",API_HOST )
    .send()
    .await?
    .json()
    .await?;

    println!("{:?}",flight_status_request);
    Ok(())
}