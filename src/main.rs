mod flight;

fn main(){
    let x = flight::Flight::get_information();
    match x {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}