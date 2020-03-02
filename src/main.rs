use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        print!("{} ", arg);
    }

}

