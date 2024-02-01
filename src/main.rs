use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //println!("{:?}", args);

    let source_path = &args[1];

    println!("{:?}", source_path);

    let mut source = std::fs::File::open(source_path).expect("File not found.");
    let mut source_contents = String::new();

    source.read_to_string(&mut source_contents).expect("Something went wrong reading the file.");

    println!("The texts:\n{}", source_contents);
}
