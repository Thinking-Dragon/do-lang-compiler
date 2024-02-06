use crate::token::Token;

pub fn tokenize(source_code_raw: String) -> Vec<Token> {
    let source_code = &source_code_raw
        .lines()
        .map(
            |line|
            line.split("//").next().unwrap_or("").trim()
        )
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_string();

    println!("Source code without comments:\n{}", source_code);

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

        if current_character.is_alphanumeric() {
            let mut symbol_name = "".to_string();
            while current_character.is_alphanumeric() {
                symbol_name.push(current_character);
                cursor += 1;
                current_character = source_code.chars().nth(cursor).unwrap();
            }
            tokens.push(Token::Symbol(symbol_name));
        }
        else {
            cursor += 1;
        }
    }

    tokens
}
