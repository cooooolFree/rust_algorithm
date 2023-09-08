fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) =>  println!("{}", input.replace("\n", "").parse::<i32>().unwrap() - 543),
        Err(err) => println!("{}",err)
    }
}
