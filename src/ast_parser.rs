use std::iter::Peekable;

use crate::token::Token;
use crate::ast::ASTNode;

pub fn parse_ast(tokens: Vec<Token>) -> ASTNode {
    let mut iterator = tokens.into_iter().peekable();
    parse_program(&mut iterator)
}

fn parse_program<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    let mut statements = Vec::new();

    while iterator.peek().is_some() {
        statements.push(parse_statement(iterator));
    }

    ASTNode::new_program(statements)
}

fn parse_statement<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    match iterator.next() {
        Some(Token::Data)  => parse_data(iterator),
        Some(Token::Group) => parse_group(iterator),
        Some(Token::Do)    => parse_do(iterator),
        Some(Token::Run)   => parse_run(iterator),
        None               => panic!("No token provided."),
        Some(unexpected)   => panic!("Unexpected token: {}", unexpected.get_value()),
    }
}

fn parse_data<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        let mut fields: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LBrace) {
            panic!("Expected left brace to open data structure body.");
        }

        iterator.next();

        while !token_is(iterator, Token::RBrace) {
            if token_is(iterator, Token::Comma) {
                iterator.next();
            }

            fields.push(parse_field(iterator));
        }

        if token_is(iterator, Token::RBrace) {
            iterator.next();
        }

        ASTNode::new_data(name_token.unwrap().get_value(), fields)
    }
    else {
        panic!("Data structure requires a name.");
    }
}

fn parse_group<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        let mut parameters: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LParenthesis) {
            panic!("Expected left parenthesis to open group parameters.");
        }

        iterator.next();

        while !token_is(iterator, Token::RParenthesis) {
            if token_is(iterator, Token::Comma) {
                iterator.next();
            }

            parameters.push(parse_parameter(iterator));
        }

        if token_is(iterator, Token::RParenthesis) {
            iterator.next();
        }

        let mut data_instanciations: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LBrace) {
            panic!("Expected left brace to open group body.");
        }

        iterator.next();

        while !token_is(iterator, Token::RBrace) {
            if token_is(iterator, Token::Comma) {
                iterator.next();
            }

            data_instanciations.push(parse_data_instanciation(iterator));
        }

        if token_is(iterator, Token::RBrace) {
            iterator.next();
        }

        ASTNode::new_group(name_token.unwrap().get_value(), parameters, data_instanciations)
    }
    else {
        panic!("Group requires a name.");
    }
}

fn parse_do<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        let mut instructions: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LBrace) {
            panic!("Expected left brace to open do body.");
        }

        iterator.next();

        while !token_is(iterator, Token::RBrace) {
            instructions.push(parse_instruction(iterator));
        }

        if token_is(iterator, Token::RBrace) {
            iterator.next();
        }

        ASTNode::new_do(name_token.unwrap().get_value(), instructions)
    }
    else {
        panic!("Do requires a name.");
    }
}

fn parse_run<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    let mut actions_to_do: Vec<String> = Vec::new();

    if !token_is(iterator, Token::LParenthesis) {
        panic!("Expected left parenthesis to open list of actions to do.");
    }

    iterator.next();

    while !token_is(iterator, Token::RParenthesis) {
        if is_symbol(&iterator.peek()) {
            let action_to_do_token = iterator.next();
            actions_to_do.push(action_to_do_token.unwrap().get_value());
        }
        else {
            iterator.next();
        }
    }

    if token_is(iterator, Token::RParenthesis) {
        iterator.next();
    }

    let mut instructions: Vec<ASTNode> = Vec::new();

    if !token_is(iterator, Token::LBrace) {
        panic!("Expected left brace to open run body.");
    }

    iterator.next();

    while !token_is(iterator, Token::RBrace) {
        instructions.push(parse_instruction(iterator));
    }

    if token_is(iterator, Token::RBrace) {
        iterator.next();
    }

    ASTNode::new_run(actions_to_do, instructions)
}

fn parse_field<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {    
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();

        if token_is(iterator, Token::Colon) {
           iterator.next(); 
        }
        else {
            panic!("Expected : before field type.");
        }
        
        if is_symbol(&iterator.peek()) {
            let field_type_token = iterator.next();
            ASTNode::new_field(name_token.unwrap().get_value(), field_type_token.unwrap().get_value())
        }
        else {
            panic!("Expected type for field with name: {}", name_token.unwrap().get_value());
        }
    }
    else {
        panic!("Expected name for field.");
    }
}

fn parse_field_value<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        if token_is(iterator, Token::Equal) {
           iterator.next(); 
        }
        else {
            panic!("Expected : before field value.");
        }

        if is_symbol(&iterator.peek()) {
            let field_type_token = iterator.next();
            ASTNode::new_field_value(name_token.unwrap().get_value(), field_type_token.unwrap().get_value())
        }
        else {
            panic!("Expected value for field: {}", name_token.unwrap().get_value());
        }
    }
    else {
        panic!("Expected name of field.");
    }
}

fn parse_parameter<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        if token_is(iterator, Token::Colon) {
           iterator.next(); 
        }
        else {
            panic!("Expected : before parameter type.");
        }

        if is_symbol(&iterator.peek()) {
            let parameter_type_token = iterator.next();
            ASTNode::new_parameter(name_token.unwrap().get_value(), parameter_type_token.unwrap().get_value())
        }
        else {
            panic!("Expected type for parameter with name: {}", name_token.unwrap().get_value());
        }
    }
    else {
        panic!("Expected name for parameter.");
    }
}

fn parse_data_instanciation<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        let mut field_values: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LParenthesis) {
            panic!("Expected left parenthesis to open data instanciation values.");
        }

        iterator.next();

        while !token_is(iterator, Token::RParenthesis) {
            if token_is(iterator, Token::Comma) {
                iterator.next();
            }

            field_values.push(parse_field_value(iterator));
        }

        if token_is(iterator, Token::RParenthesis) {
            iterator.next();
        }

        ASTNode::new_data_instanciation(name_token.unwrap().get_value(), field_values)
    }
    else {
        panic!("Expected name of data structure to instanciate.");
    }
}

fn parse_instruction<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    if token_is(iterator, Token::CreateInstructionKeyword) {
        return parse_create_instruction(iterator);
    }

    parse_create_instruction(iterator)
}

fn parse_create_instruction<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    iterator.next();

    if is_symbol(&iterator.peek()) {
        let name_token = iterator.next();
        let mut parameter_values: Vec<ASTNode> = Vec::new();

        if !token_is(iterator, Token::LParenthesis) {
            panic!("Expected left parenthesis to open group creation parameters.");
        }

        iterator.next();

        while !token_is(iterator, Token::RParenthesis) {
            if is_symbol(&iterator.peek()) {
                parameter_values.push(parse_value(iterator));
            }
            else {
                iterator.next();
            }
        }

        if token_is(iterator, Token::RParenthesis) {
            iterator.next();
        }

        ASTNode::new_create_instruction(name_token.unwrap().get_value(), parameter_values)
    }
    else {
        panic!("Expected the name of a group to create.");
    }
}

fn parse_value<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>) -> ASTNode {
    let mut value: String = "".to_string();

    if is_symbol(&iterator.peek()) {
        value += iterator.next().unwrap().get_value().as_str();
    }

    if token_is(iterator, Token::Dot) {
        value += Token::Dot.get_value().as_str();

        iterator.next();
        if is_symbol(&iterator.peek()) {
            value += iterator.next().unwrap().get_value().as_str();
        }
    }

    ASTNode::new_value(value)
}

fn token_is<I: Iterator<Item=Token>>(iterator: &mut Peekable<I>, token: Token) -> bool {
    iterator.peek().is_some() && iterator.peek().unwrap() == &token
}

fn is_symbol(token: &Option<&Token>) -> bool {
    if let Some(Token::Symbol(_)) = token {
        true
    }
    else {
        false
    }
}
