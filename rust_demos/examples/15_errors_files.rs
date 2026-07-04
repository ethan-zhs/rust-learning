use std::fs;
use std::io;
use std::path::PathBuf;

fn demo_file_path() -> PathBuf {
    std::env::temp_dir().join("rust_demo_name.txt")
}

fn save_name(name: &str) -> io::Result<PathBuf> {
    let path = demo_file_path();
    fs::write(&path, name)?;
    Ok(path)
}

fn load_name(path: &PathBuf) -> io::Result<String> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().to_string())
}

fn main() -> io::Result<()> {
    let path = save_name("Alice")?;
    let name = load_name(&path)?;
    println!("loaded name = {name}");
    println!("file path = {}", path.display());
    Ok(())
}
