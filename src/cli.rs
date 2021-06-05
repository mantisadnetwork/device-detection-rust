#![allow(warnings)]

use fiftyonedegrees::hash::get_device;

fn main() {
    let agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".to_string();

    get_device(agent);

    println!("yay");
}