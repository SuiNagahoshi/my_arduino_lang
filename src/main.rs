use std::io::Read;

/*enum Token {
    NAME,
    PRINTLN,
    RETURN,

    LBRACKET,
    RBRACKET,
    LCURLY,
    RCURLY,
    LSQUARE,
    RSQUARE,

    DQUOTATION,
    SQUOTATION,

    TRUE,
    FALSE,
}*/

fn lex(content: String)/* -> Vec<(Token, String)>*/ {
    //let mut token_sets: Vec<(Token, String)> = Vec::new();
    
    //let mut tokens: Vec<&str> = content.split_whitespace().collect::<Vec<&str>>();

    //println!("{:?}", tokens);

    let mut tmp_token: String = "".to_string();
    let mut tokens: Vec<String> = Vec::new();

    for t in content.as_str().chars() {
        tmp_token.push(t);
        match t {
            ' ' => {
                tokens.push(tmp_token);
                tmp_token = "".to_string();
            },
            '\n' => {
                tokens.push(tmp_token);
                tmp_token = "".to_string();
            },
            '\"' => {
                tokens.push(tmp_token);
                tmp_token = "".to_string();
            },
            _ => ()
        }
    }

    println!("{:?}", tokens);

}

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    //println!("{:?}", args);

    let source_path = &args[1];

    println!("{:?}", source_path);

    let mut source = std::fs::File::open(source_path).expect("File not found.");
    let mut source_contents = String::new();

    source.read_to_string(&mut source_contents).expect("Something went wrong reading the file.");

    println!("The texts:\n{}", source_contents);

    lex(source_contents);
}
