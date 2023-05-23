use std::{env, error::Error, fs, io, path::Path};

fn main() {
    let mut args = env::args();
    args.next();

    for x in args {
        match x.as_str() {
            "zen" => match read_prompt() {
                Ok(read_prompt) => read_prompt,
                Err(e) => eprintln!("ERROR:{e}"),
            },
            _ => match read_from_file(x) {
                Ok(read_from_file) => read_from_file,
                Err(e) => eprintln!("ERROR:{e}"),
            },
        }
    }
}

fn read_prompt() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{buffer}");
    Ok(())
}

fn read_from_file(x: String) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(x.as_str());
    let file_content = fs::read_to_string(file_path)?;
    let lines = file_content.split("\n").collect::<Vec<_>>();
    println!("{lines:?}");
    Ok(())
}
