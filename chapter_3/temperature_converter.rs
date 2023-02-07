//  A code to convert temperatures between Fahrenheit and Celsius

use std::io::stdin;

fn fahr_to_cel(number:  i16) ->  i16 {
    ((number - 32) * 5) / 9
}

fn cel_to_fahr(number: i16) -> i16 {
    number * 9 / 5 + 32
}


fn main() {
    loop {
        println!("Do you wish to convert Fahrenheit or Celsius (F/C)?");
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read the line.");
        println!("Input the number:");
        let mut number = String::new();
        stdin().read_line(&mut number).expect("Failed to read the line.");
        let number: i16 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) =>  {
                    println!("Error, please input a number.");
                    continue;
                },
        };
        if choice.trim().to_lowercase() == "f" {    
            println!("{} Fahrenheit = {} celcius", number, fahr_to_cel(number));
        } else if choice.trim().to_lowercase() == "c" {
            println!("{} celsius = {} Fahrenheit", number, cel_to_fahr(number));
        } else {
            println!("Please only answer F or C.");
            continue;
        }
        choice.clear();
        println!("Continue using? (Y/N):");
        stdin().read_line(&mut choice).expect("Failed to read line.");
        if choice.trim().to_lowercase() == "n" {
            break;
        } else if choice.trim().to_lowercase() == "y" {
            continue;
        } else {
            println!("Unexpected answer, terminating.");
            break;
        }
    }
}
