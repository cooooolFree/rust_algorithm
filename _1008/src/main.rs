fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut numbers = input.split_whitespace().flat_map(str::parse::<f64>);
    let a:f64 = numbers.next().unwrap();
    let b:f64 = numbers.next().unwrap();

    println!("{}", a / b)
}
