#![allow(non_snake_case)]

mod flight;
mod airport;

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

    // get airport information by airport code
    let airport_by_code = airport::Airport::get_airport_data_by_code();
    match airport_by_code {
        Err(e) => println!("{:?}", e),
        _ => ()
    }

    // get distance between connecting flights
    let distance_of_connecting_flights = airport::Airport::get_distance_between_two_flights();
    if distance_of_connecting_flights.is_ok(){
        println!("{:#?}",distance_of_connecting_flights);
    }
}
