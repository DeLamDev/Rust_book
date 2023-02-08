use std::io;

fn fibonacci(number: u16) -> u16 {
    
    if number <= 1 {
        return number;
    }

    let mut previous: u16 = 0;
    let mut current: u16 = 1;

    for _ in 0..(number - 1) {
        let new: u16 = previous + current;
        previous = current;
        current = new;
    }
    current
}

fn main() {
    println!("Input the desired nth fibonacci number: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line.");
    let choice: u16 = choice.trim().parse().expect("Failed to read line.");
    println!("The result is: {}", fibonacci(choice));
}
