fn main() {
    for i in 0..usize::MAX {
        if i % 5 == 0 {
            println!("Buzz");
        } else if i % 7 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 && i % 7 == 0 {
            println!("Buzz / Fizz");
        } else {
            println!("{}", i);
        }
    }
}
