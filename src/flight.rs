pub mod status;

use status::FlightData;


#[derive(Default)]
pub struct Flight {
    pub status:FlightData,
}

impl Flight {
    fn new() -> Flight{
         Flight{
           ..Default::default()
    }
}
