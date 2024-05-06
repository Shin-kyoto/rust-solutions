use clap::App;

fn main() {
    let _matches = App::new("echor").version("0.1.0").about("Rust echo").author("Shintaro Tomie").get_matches();
    // println!("{:?}", std::env::args());
    // println!("Hello, world!");
    println!("{:#?}", _matches);
}
