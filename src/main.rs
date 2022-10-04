#![allow(non_snake_case)]

mod flight;

fn main(){

    // retrieves all the information regarding a flight
    let flight_data_information = flight::Flight::get_information();
    match flight_data_information {
        Err(e) => println!("{:?}", e),
        _ => ()
    }

    // retrieves an information regarding flight departures
    let flight_departures_information = flight::Flight::flight_departures();
    match flight_departures_information {
        Err(e) => println!("{:?}",e),
        _ => (),
    }
}
