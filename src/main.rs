use std::{env, fs, io, path::Path};

fn main() {
    let mut args = env::args();
    args.next();

    for x in args {
        match x.as_str() {
            "zen" => read_prompt(),
            _ => read_from_file(x),
        }
    }
}

fn read_prompt() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn read_from_file(x: String) {
    let file_path = Path::new(x.as_str());
    let file_content = fs::read_to_string(file_path).expect("ERROR: no file found");
    let _lines = file_content.split("\n").collect::<Vec<_>>();
}
