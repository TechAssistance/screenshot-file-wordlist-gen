fn main() {
    let characters: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    let length = 6;
    let num_characters = characters.len();
    let num_combinations = num_characters.pow(length as u32);

    for i in 0..num_combinations {
        let mut bitmask = i;
        let mut string = String::with_capacity(length);

        for _ in 0..length {
            let index = bitmask % num_characters;
            let character = characters[index];

            if let Some(last_char) = string.chars().last() {
                if character == last_char {
                    bitmask += 1;
                    continue;
                }
            }

            string.push(character);
            bitmask /= num_characters;
        }

        if string.len() == length {
            println!("{}.png", string);
        }
    }
}

