use walkdir::WalkDir;

fn main() {
    let start_dir = "/Users/fmgono/Projects";
    let walker = WalkDir::new(start_dir)
        .into_iter()
        .filter_entry(|e| e.file_name() != "node_modules" && e.file_name() != ".next");

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_name() == "package.json" {
            if let Some(parent) = entry.path().parent() {
                if let Some(project_name) = parent.file_name() {
                    println!("{}", project_name.to_string_lossy());
                } else {
                    eprintln!("Error: Could not get parent directory name");
                }
            } else {
                eprintln!("Error: Could not get parent directory");
            }
        }
    }
}