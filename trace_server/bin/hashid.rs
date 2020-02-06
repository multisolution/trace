use std::str::FromStr;

fn main() {
    let harsh = harsh::HarshBuilder::new().init().unwrap();

    let mut args = std::env::args();

    let ids = args.nth(1).unwrap();
    let ids: Vec<&str> = ids.split(",").collect();
    let ids: Vec<u64> = ids.iter().map(|id| u64::from_str(id).unwrap()).collect();

    println!("{}", harsh.encode(ids.as_ref()).unwrap());
}