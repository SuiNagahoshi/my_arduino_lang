use std::io::Read;

fn replace_tab(content: String) -> String {
    let replaced = content.replace("    ", "\t");
    replaced
}

fn replace_tokens(content: String) -> String {
    let to_replace_token = ["\n" ,"\t", "(", ")", "[", "]", "{", "}", "->"];
    let mut token_set = content;
    for r in to_replace_token {
        token_set = token_set.replace(r, &format!(" {} ", r));
    }
    token_set
}

fn split_spaces(content: String) -> Vec<String> {
    //let mut token_set:Vec<&str> = content.split(' ').collect();
    let mut token_set = content.split(' ').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    /*for i in contents {
        if &i != &"" {
            tokens.push(i);
        }
    }*/
    token_set.retain(|x| x != "");
    token_set
}

fn lex(content: String) -> Vec<String> {
    
    let mut replaced_content = replace_tab(content);
    println!("{}", replaced_content);

    replaced_content = replace_tokens(replaced_content);
    println!("{}", replaced_content);
    
    //let tmp_contents: Vec<&str> = replaced_content.split(' ').collect();//.split_whitespace().collect::<Vec<&str>>();
    //tmp_contents.retain(|&x| x == "");
    let tokens = split_spaces(replaced_content);//Vec::new();
    println!("{:?}", tokens);
    //let lexed_content = tmp_contents;
    tokens
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

    //let tokens = lex2(source_contents);

    println!("{:?}", lex(source_contents));
}
