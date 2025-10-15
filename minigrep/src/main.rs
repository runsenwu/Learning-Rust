use minigrep::Config;
use std::env;
use std::process;


fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    println!("After calling closure: {list:?}");
    borrows_mutably();

    borrows_mutably();
}
// fn main() {
//     // 1. take command line arguments
//     let args: Vec<String> = env::args().collect();

//     let configs = Config::build(&args).unwrap_or_else(|err: &'static str| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1)
//     });

//     // 2. how to read a file
//     if let Err(error) = minigrep::run(configs) {
//         eprintln!("Error opening the file: {error}");
//         process::exit(1)
//     }

//     // 3. how to search through the file
// }
