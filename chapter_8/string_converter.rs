// Convert strings to pig latin
// First consonant is moved to the end and "ay" is added after
// If vowel then "h" is moved at the end and "ay" added after
// Keep in mind details of UTF-8 enconding


fn consonant(word: String) -> String {
    let mut characters = word.chars();
    let final_letter = characters[0];
    characters.next();
    characters.push("-" + final_letter + "ay");
    return characters.as_str() as String
}

fn vowel(word: String) -> String {}


fn main() {
    let example = String::from("Test");
    println!("Resuktado: {}", consonant(example));
}
