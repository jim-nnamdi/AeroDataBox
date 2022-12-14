use chrono::{ NaiveDateTime };

use reqwest::{ self, Error };
use serde::{ Serialize, Deserialize };
use dotenv::dotenv;

/// Declare module variables to be used

pub const API_URL: &str = "https://aerodatabox.p.rapidapi.com/flights/number/KL1395/2022-09-30";

#[derive(Debug, Serialize, Deserialize)]
pub struct Aircraft {
    pub model: String,
    pub reg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airline {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airport {
    #[serde(rename = "countryCode")]
    pub country_code: String,

    pub iata: String,
    pub icao: String,
    pub location: Location,

    #[serde(rename = "municipalityName")]
    pub municipality_name: String,

    pub name: String,

    #[serde(rename = "shortName")]
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    pub basic: String,
    pub live: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrival {
    #[serde(rename = "actualTimeLocal")]
    pub actual_time_local: NaiveDateTime,

    #[serde(rename = "actualTimeUTC")]
    pub actual_time_utc: NaiveDateTime,

    pub airport: Airport,

    #[serde(rename = "baggageBelt")]
    pub baggage_belt: String,
    pub gate: String,
    pub quality: Vec<Quality>,

    #[serde(rename = "scheduledTimeLocal")]
    pub scheduled_time_local: NaiveDateTime,

    #[serde(rename = "scheduledTimeUTC")]
    pub scheduled_time_utc: NaiveDateTime,
    pub terminal: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightData {
    pub aircraft: Aircraft,
    pub airline: Airline,
    pub arrival: Arrival,

    #[serde(rename = "camelcase")]
    pub callsign: String,

    #[serde(rename = "codeshareStatus")]
    pub code_share_status: String,

    #[serde(rename = "isCargo")]
    pub is_cargo: bool,
    pub status: String,
}

impl FlightData {
    #[tokio::main]
    pub async fn flight_information() -> Result<(), Error> {
      dotenv().ok();
        let aerobox_client = reqwest::Client::new();

        let flight_status_request: Vec<FlightData> = aerobox_client
            .get(API_URL)
            .header("X-RapidAPI-Key", std::env::var("APIKEY").expect("api key is required"))
            .header("X-RapidAPI-Host", std::env::var("APIHOST").expect("please cross check the host"))
            .send().await?
            .json().await?;

        println!("{:#?}", flight_status_request);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_retrieve_flight_information() -> Result<(), Error> {
      let result = FlightData::flight_information();
      match result {
         Err(e) => {
            Err(e)
         }
         _ => Ok(()),
      }
    }
}