fn main() {
    for arg in &std::env::args().collect::<Vec<String>>()[1..] {
        print!("{} ", arg);
    }
}

