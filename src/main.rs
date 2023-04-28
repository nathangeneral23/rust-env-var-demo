use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let msg = std::env::var("MESSAGE");

    match msg {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err)
    }
}
