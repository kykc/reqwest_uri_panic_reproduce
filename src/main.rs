extern crate reqwest;

fn main() {
    let uri_str = "file:///etc/resolv.conf";
    let mut _resp = reqwest::get(uri_str);
    println!("Hello, world!");
}
