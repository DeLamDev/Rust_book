//  A code to convert temperatures between Fahrenheit and Celsius

use std::io::stdin;

fn fahrenheit_celsius(number:  i16) ->  i16 {
    (number - 32) * 5/9
}

fn celsius_fahrenheit(number: i16) -> i16 {
    number * 9/5 + 32
}


fn main() {
    loop {
        let mut cont_choice = String::new();
        println!("Do you wish to convert Fahrenheit or Celsius (F/C)?");
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read the line.");
        let choice: String = match choice.trim().to_lowercase().as_str() {
            "f" => "f".to_string(),
            "c" => "c".to_string(),
            _ => {
                println!("Please enter either F or C.");
                continue;
            }
        };
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
        if choice == "f" {    
            println!("{} Fahrenheit = {} celcius", number, fahrenheit_celsius(number));
        } else if choice.trim().to_lowercase() == "c" {
            println!("{} celsius = {} Fahrenheit", number, celsius_fahrenheit(number));
        }
        println!("Continue using? (Y/N):");
        stdin().read_line(&mut cont_choice).expect("Failed to read line.");
        if cont_choice.trim().to_lowercase() == "n" {
            break;
        } else if cont_choice.trim().to_lowercase() == "y" {
            continue;
        } else {
            println!("Unexpected answer, terminating.");
            break;
        }
    }
}
