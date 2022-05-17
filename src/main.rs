use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new(".");
    show_entries(path)
}

pub fn show_entries(path: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        println!("{:?}", entry_path);
        if entry_path.is_dir() {
            show_entries(&entry_path)?;
        }
    }
    Ok(())
}
