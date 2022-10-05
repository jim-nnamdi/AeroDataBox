use reqwest::{self,Error};
use serde::{Serialize, Deserialize};

pub const API_KEY: &str = "APIKEY";
pub const API_HOST: &str = "APIHOST";
pub const BASE_URL: &str ="https://aerodatabox.p.rapidapi.com/airports/%7BcodeType%7D/LHR/distance-time/LAX";

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromAirport {
    pub icao: String,
    pub iata: String,
    pub name: String,

    #[serde(rename="shortName")]
    pub shortname: String,
    pub location: Location
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ToAirport{
    pub icao: String,
    pub iata: String,
    pub name: String,

    #[serde(rename = "shortName")]
    pub shortname: String,

    #[serde(rename = "muncipalityName")]
    pub muncipalityname: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GreatCircleDistance{
    pub meter: u64,
    pub km: u64,
    pub mile: u64,
    pub nm: u64,
    pub feet: u64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Distance{
    pub from: FromAirport,
    pub to: ToAirport,

    #[serde(rename="greatCircleDistance")]
    pub greatcircledistance: GreatCircleDistance
}

impl Distance{

    #[tokio::main]
    pub async fn distance_between_two_airport_flights() -> Result<Distance,Error> {
        let client = reqwest::Client::new();
        let distance_data = client.get(BASE_URL)
        .header("X-RapidAPI-Key", API_KEY)
        .header("X-RapidAPI-Host", API_HOST)
        .send().await
        .expect("retrieving distance ...")
        .json::<Distance>().await;
        return distance_data;
    }
}