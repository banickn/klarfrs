use std::path::PathBuf;
use klarfrs::parse;

fn main() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("klarf01.txt");
    
    let klarfrs = parse(path.to_str().unwrap()).unwrap();

    println!("{:#?}", klarfrs);

    Ok(())
}