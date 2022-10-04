pub mod status;
pub mod departures;

use chrono::{ NaiveDateTime, NaiveDate, NaiveTime };
use status::FlightData;
use departures::Departures;

use self::status::{ Aircraft, Airline, Arrival, Airport, Location, Quality };

pub struct Flight {
    pub status: FlightData,
}

impl Flight {
    #[allow(dead_code)]
    pub fn new(&self) -> Flight {
        let flight_info = Flight {
            status: FlightData {
                aircraft: Aircraft {
                    model: "boeing".to_string(),
                    reg: "KLMN09".to_string(),
                },
                airline: Airline {
                    name: "airpeace".to_string(),
                },
                arrival: Arrival {
                    actual_time_local: NaiveDateTime::new(
                        NaiveDate::from_ymd(2015, 6, 3),
                        NaiveTime::from_hms_milli(12, 34, 56, 789)
                    ),
                    actual_time_utc: NaiveDateTime::new(
                        NaiveDate::from_ymd(2015, 6, 3),
                        NaiveTime::from_hms_milli(12, 34, 56, 789)
                    ),
                    airport: Airport {
                        country_code: "+234".to_string(),
                        iata: "iata".to_string(),
                        icao: "icao".to_string(),
                        location: Location {
                            lat: 89.0004,
                            lon: 90.0003,
                        },
                        municipality_name: "mname".to_string(),
                        name: "MMA2".to_string(),
                        short_name: "MMA2".to_string(),
                    },
                    baggage_belt: "belt".to_string(),
                    gate: "2nd_gate".to_string(),
                    quality: vec![Quality {
                        basic: "Basic".to_string(),
                        live: "Live".to_string(),
                    }],
                    scheduled_time_local: NaiveDateTime::new(
                        NaiveDate::from_ymd(2015, 6, 3),
                        NaiveTime::from_hms_milli(12, 34, 56, 789)
                    ),
                    scheduled_time_utc: NaiveDateTime::new(
                        NaiveDate::from_ymd(2015, 6, 3),
                        NaiveTime::from_hms_milli(12, 34, 56, 789)
                    ),
                    terminal: "MMA".to_string(),
                },
                callsign: "auto_data".to_string(),
                code_share_status: "KLMN09".to_string(),
                is_cargo: false,
                status: "takeoff".to_string(),
            },
        };

        flight_info
    }

    #[allow(dead_code)]
    pub fn get_information() -> Result<(), reqwest::Error> {
        let f_info = FlightData::flight_information();
        match f_info {
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }

    #[allow(dead_code)]
    pub fn flight_departures() -> Result<(), reqwest::Error> {
        let f_departures = Departures::<String>::flight_departure_dates();
        match f_departures {
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }
}