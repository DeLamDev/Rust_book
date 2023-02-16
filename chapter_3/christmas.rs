


fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
    "nineth", "tenth", "eleventh", "twelfth"];
    
    let chorus = "On the {} day of Christmas my true love sent to me";

    let numbers = ["", "Two", "Three", "Four", "Five", "Six", "Seven",
    "Eight", "Nine", "Ten", "Eleven", "Twelve"];

    let gifts = ["{}A patridge in a pear tree", "{} turtle doves, and",
    "{} french hens", "{} calling birds", "{} golden rings", "{} geese a-laying",
    "{} swans a-swimming", "{} maids a-milking", "{} ladies dancing",
    "{} lords a-leaping", "{} pipers piping", "{} drummers drumming"];
    
    let mut counter: u8 = 0; 

    for day in days {
        println!(chorus, day);
    }
}
