mod parser;

fn main() {
    let path = "model.obj";
    let data = std::fs::read_to_string(path).expect("Unable to read file");
    let parsed = parser::parse(&data).expect("Failed to parse the .obj file");
    println!("{:?}", parsed);
}
