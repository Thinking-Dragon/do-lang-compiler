use crate::token::Token;

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut cursor = 0;
    while cursor < source_code.len() {
        let mut current_character = source_code.chars().nth(cursor).unwrap();

        if current_character == ' ' {
            cursor += 1;
            continue;
        }

        for possible_token in Token::iter_static() {
            let left = cursor;
            let right = cursor + possible_token.get_value().len();

            if right > source_code.len() {
                continue;
            }

            if &source_code[left..right] == possible_token.get_value() {
                tokens.push(possible_token.clone());
                
                if right < source_code.len() {
                    cursor = right;
                    current_character = source_code.chars().nth(cursor).unwrap();
                }
            }
        }

        let mut added_dynamic_token = false;

        if current_character.is_alphabetic() {
            let mut symbol_name = "".to_string();
            while current_character.is_alphanumeric() {
                symbol_name.push(current_character);
                cursor += 1;
                current_character = source_code.chars().nth(cursor).unwrap();
            }
            tokens.push(Token::Symbol(symbol_name));
            added_dynamic_token = true;
        }

        if current_character.is_digit(10) {
            let mut number_value = "".to_string();
            while current_character.is_digit(10) {
                number_value.push(current_character);
                cursor += 1;
                current_character = source_code.chars().nth(cursor).unwrap();
            }
            tokens.push(Token::Integer(number_value.parse::<i64>().unwrap()));
            added_dynamic_token = true;
        }

        if !added_dynamic_token {
            cursor += 1;
        }
    }

    tokens
}
