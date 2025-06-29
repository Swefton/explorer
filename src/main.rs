use std::fs;

fn main() -> std::io::Result<()> {
    let path = "C:\\Users";

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}
