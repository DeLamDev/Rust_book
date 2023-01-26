// Given a list of integers, use a vector and return the mean, median and mode.
use std::collections::HashMap;

fn mean(list: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for number in list {
        counter += *number;
    }
    return counter / list.len() as i32;
}

fn median(list: &mut Vec<i32>) -> i32{
    list.sort();
    let len = list.len();
    if len % 2 == 0 {
        return (list[len / 2 -1] + list[len / 2]) / 2
    } else {
        return list[len / 2]
    }
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut counter: i32 = 1;
    let mut result = 0;
    let mut count_register = vec![];
    for number in list {
        let element = map.entry(number).or_insert(0);
        *element += 1;
    }
    // Handles the posibility of having two numbers that repeat the same number of times
    for (_, repetitions) in &map {count_register.push(repetitions)}
    count_register.sort();
    println!("{:?}", count_register);
    if count_register[count_register.len() - 1] == count_register[count_register.len() - 2] {return result}
    // Handles the case that no number repeats or there is one that does
    for (number, rep) in map {
        if rep > counter {
            result = *number;
            counter = rep;
        }
    }
    return result
}


fn main() {
    // Be sure to change the vector so that you can experiment
    let mut num_list: Vec<i32> = vec![12,34,16,12,32,40,12];
    println!("The mean is: {}", mean(&num_list));
    println!("The median is: {}", median(&mut num_list));
    println!("The mode is: {}", mode(&num_list));

}
