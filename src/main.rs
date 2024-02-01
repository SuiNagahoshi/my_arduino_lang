fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let source_path = &args[1];

    println!("{:?}", source_path);
}
