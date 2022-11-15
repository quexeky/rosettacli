const SEPARATOR: char = '|';
const ESCAPE: char = '^';

pub fn parse_line(line: &String) -> Vec<String>{
    let mut token = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut characters = line.chars();

    while let Some(ch) = characters.next() {
        match ch {
            SEPARATOR => {
                tokens.push(token);
                token = String::new();
            },
            ESCAPE => {
                if let Some(next) = characters.next() {
                    if next == ESCAPE { token.push(ch); continue; }
                    token.push(next);
                }
            },
            _ => token.push(ch),
        }
    }
    tokens.push(token);
    println!("{:?}", tokens);
    return tokens;

}