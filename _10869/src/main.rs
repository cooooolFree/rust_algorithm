fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut numbers = input.split_whitespace().flat_map(str::parse::<i32>);
    let a:i32 = numbers.next().unwrap();
    let b:i32 = numbers.next().unwrap();

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", a % b);
}
