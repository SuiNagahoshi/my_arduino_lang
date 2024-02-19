use std::io::Read;

use unicode_segmentation::UnicodeSegmentation;

fn lex(content: String) -> Vec<&'static str> {
    let to_replace_token = ["\t", "(", ")", "[", "]", "{", "}", "->"];
    let replaced_content: String = content.replace("    ", "\t");
    for r in to_replace_token {
        replaced_content.replace(r, &format!(" {} ", r));
    }
    
    let mut tmp_contents: Vec<&str> = replaced_content.split(' ').collect::<Vec<&str>>();//.split_whitespace().collect::<Vec<&str>>();
    tmp_contents.retain(|&x| x == "");
    //let lexed_content = tmp_contents;
    return tmp_contents;
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
