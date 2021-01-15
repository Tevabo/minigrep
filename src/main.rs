use std::env;

fn main() {
    let vars: Vec<String> = env::args().collect();
    println!("Vars: {:?}", vars)
}
