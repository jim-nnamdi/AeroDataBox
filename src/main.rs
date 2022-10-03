mod flight;

fn main() {
    let foo = flight::flight();
    match foo {
        Err(e) => println!("{:?}",e),
        _ => ()
    }
}