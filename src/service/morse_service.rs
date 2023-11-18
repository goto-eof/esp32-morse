use std::collections::HashMap;

pub const SPACE_BETWEEN_CHARS: u32 = 50;
const SHORT: u32 = 100;
const LONG: u32 = 300;
const PAUSE: u32 = 500;
const END: u32 = 2000;

pub fn translate(string: &str) -> Result<Vec<u32>, String> {
    let translation = retrieve_translation();

    let mut morse_result: Vec<u32> = Vec::new();
    for character in string.to_uppercase().chars() {
        let morse_character_result = translation.get(&character);
        if morse_character_result.is_none() {
            let err_message = format!(
                "Invalid character: could not translate the character: {}",
                character
            );
            return Err(err_message);
        }
        let morse = &mut morse_character_result.unwrap().to_owned();
        morse_result.append(morse);
        morse_result.append(&mut vec![PAUSE])
    }
    morse_result.append(&mut vec![END]);
    return Ok(morse_result);
}

fn retrieve_translation() -> HashMap<char, Vec<u32>> {
    let translation: HashMap<char, Vec<u32>> = HashMap::from([
        ('A', vec![SHORT, LONG]),
        ('B', vec![LONG, SHORT, SHORT, SHORT]),
        ('C', vec![LONG, SHORT, LONG, SHORT]),
        ('D', vec![LONG, SHORT, SHORT]),
        ('E', vec![SHORT]),
        ('F', vec![SHORT, SHORT, LONG, SHORT]),
        ('G', vec![LONG, LONG, SHORT]),
        ('H', vec![SHORT, SHORT, SHORT, SHORT]),
        ('I', vec![SHORT, SHORT]),
        ('J', vec![SHORT, LONG, LONG, LONG]),
        ('K', vec![LONG, SHORT, LONG]),
        ('L', vec![SHORT, LONG, SHORT, SHORT]),
        ('M', vec![LONG, LONG]),
        ('N', vec![LONG, SHORT]),
        ('O', vec![LONG, LONG, LONG]),
        ('P', vec![SHORT, LONG, LONG, SHORT]),
        ('Q', vec![LONG, LONG, SHORT, LONG]),
        ('R', vec![SHORT, LONG, SHORT]),
        ('S', vec![SHORT, SHORT, SHORT]),
        ('T', vec![LONG]),
        ('U', vec![SHORT, SHORT, LONG]),
        ('V', vec![SHORT, SHORT, SHORT, LONG]),
        ('U', vec![SHORT, LONG, LONG]),
        ('X', vec![LONG, SHORT, SHORT, LONG]),
        ('Y', vec![LONG, SHORT, LONG, LONG]),
        ('Z', vec![LONG, LONG, SHORT, SHORT]),
        ('1', vec![SHORT, LONG, LONG, LONG, LONG]),
        ('2', vec![SHORT, SHORT, LONG, LONG, LONG]),
        ('3', vec![SHORT, SHORT, SHORT, LONG, LONG]),
        ('4', vec![SHORT, SHORT, SHORT, SHORT, LONG]),
        ('5', vec![SHORT, SHORT, SHORT, SHORT, SHORT]),
        ('6', vec![LONG, SHORT, SHORT, SHORT, SHORT]),
        ('7', vec![LONG, LONG, SHORT, SHORT, SHORT]),
        ('8', vec![LONG, LONG, LONG, SHORT, SHORT]),
        ('9', vec![LONG, LONG, LONG, LONG, SHORT]),
        ('0', vec![LONG, LONG, LONG, LONG, LONG]),
        (' ', vec![PAUSE, PAUSE]),
    ]);
    translation
}