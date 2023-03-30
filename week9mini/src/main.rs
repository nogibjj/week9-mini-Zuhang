fn main() {
    // Define the Morse code table as a Rust hashmap
    let morse_table = [
        ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."), ('F', "..-."), ('G', "--."),
        ('H', "...."), ('I', ".."), ('J', ".---"), ('K', "-.-"), ('L', ".-.."), ('M', "--"), ('N', "-."),
        ('O', "---"), ('P', ".--."), ('Q', "--.-"), ('R', ".-."), ('S', "..."), ('T', "-"), ('U', "..-"),
        ('V', "...-"), ('W', ".--"), ('X', "-..-"), ('Y', "-.--"), ('Z', "--.."), ('0', "-----"), ('1', ".----"),
        ('2', "..---"), ('3', "...--"), ('4', "....-"), ('5', "....."), ('6', "-...."), ('7', "--..."),
        ('8', "---.."), ('9', "----."), (' ', "/")
    ].iter().cloned().collect::<std::collections::HashMap<_,_>>();

    // Prompt the user to enter some text
    println!("Enter some text to translate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Translate the input to Morse code
    let morse_code = input.trim().chars().flat_map(|c| morse_table.get(&c.to_ascii_uppercase()).cloned()).collect::<Vec<_>>();
    println!("Morse code: {}", morse_code.join(" "));

    // Prompt the user to enter some Morse code
    println!("Enter some Morse code to translate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Translate the Morse code to text
    let text = input.trim().split(' ').filter_map(|s| morse_table.iter().find(|(_, &code)| code == s).map(|(&c, _)| c)).collect::<String>();
    println!("Text: {}", text);
}
