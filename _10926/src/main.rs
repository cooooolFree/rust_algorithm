fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => println!("{}??!", input.replace("\n", "")),
        Err(_err) => println!("error")
    }
}
