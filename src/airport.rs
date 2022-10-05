pub mod airport_by_code;
pub mod distance_time_by_airport;

use reqwest::{self, Error};
use airport_by_code::AirportData;
use distance_time_by_airport::Distance;

pub struct Airport{
    pub airport_by_code: AirportData,
}

impl Airport{
    pub fn get_airport_data_by_code() -> Result<(), Error> {
        let airport_data_using_code = AirportData::get_airport_by_code();
        match airport_data_using_code {
            Err(e) => Err(e),
            _ => Ok(())
        }
    }

    pub fn get_distance_between_two_flights() -> Result<Distance, Error> {
        let distance_between_two_connecting_flights = Distance::distance_between_two_airport_flights();
        return distance_between_two_connecting_flights;
    }
}