fn main() {
    for num in 0..100 {
        match (num % 3, num % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => println!("{}", num),
        }
    }
}
