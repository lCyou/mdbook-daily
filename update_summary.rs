use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Get display name from filename
fn get_display_name(filename: &str) -> Option<String> {
    let name = filename.strip_suffix(".md").unwrap_or(filename);
    if name == "README" {
        None
    } else {
        Some(name.to_string())
    }
}

/// Recursively process directory and generate SUMMARY entries
fn process_directory(base_path: &Path, dir_path: &Path, level: usize) -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let indent = "  ".repeat(level);
    
    // Get all items in directory
    let mut entries: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|e| e.ok())
        .collect();
    
    // Sort entries by name
    entries.sort_by_key(|e| e.file_name());
    
    // Separate files and directories
    let mut md_files = Vec::new();
    let mut subdirs = Vec::new();
    
    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if path.is_file() && file_name_str.ends_with(".md") && file_name_str != "SUMMARY.md" {
            md_files.push(path);
        } else if path.is_dir() {
            subdirs.push(path);
        }
    }
    
    // Process subdirectories
    for subdir in subdirs {
        let dir_name = subdir.file_name().unwrap().to_string_lossy();
        let readme_path = subdir.join("README.md");
        
        if readme_path.exists() {
            let relative_path = readme_path.strip_prefix(base_path).unwrap();
            let relative_path_str = relative_path.to_str().unwrap().replace('\\', "/");
            lines.push(format!("{}- [{}](./{})", indent, dir_name, relative_path_str));
            
            // Process files in subdirectory with increased indentation
            if let Ok(subdir_lines) = process_directory(base_path, &subdir, level + 1) {
                lines.extend(subdir_lines);
            }
        } else {
            // If no README, still process subdirectory
            lines.push(format!("{}- [{}]", indent, dir_name));
            if let Ok(subdir_lines) = process_directory(base_path, &subdir, level + 1) {
                lines.extend(subdir_lines);
            }
        }
    }
    
    // Process markdown files (excluding README.md as it's already processed)
    for md_file in md_files {
        let file_name = md_file.file_name().unwrap().to_string_lossy();
        if file_name == "README.md" {
            continue;
        }
        
        if let Some(display_name) = get_display_name(&file_name) {
            let relative_path = md_file.strip_prefix(base_path).unwrap();
            let relative_path_str = relative_path.to_str().unwrap().replace('\\', "/");
            lines.push(format!("{}- [{}](./{})", indent, display_name, relative_path_str));
        }
    }
    
    Ok(lines)
}

/// Generate SUMMARY.md content from src directory structure
fn generate_summary(src_path: &Path) -> io::Result<String> {
    let mut lines = vec!["# Summary".to_string(), String::new()];
    
    // Add aboutMe.md at the top
    let about_me = src_path.join("aboutMe.md");
    if about_me.exists() {
        lines.push("- [about me](./aboutMe.md)".to_string());
        lines.push(String::new());
    }
    
    // Get all subdirectories
    let mut subdirs: Vec<_> = fs::read_dir(src_path)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    
    subdirs.sort();
    
    for subdir in subdirs {
        // Create section header
        let dir_name = subdir.file_name().unwrap().to_string_lossy();
        let section_name = capitalize_first(&dir_name);
        lines.push(format!("# {}", section_name));
        lines.push(String::new());
        
        // Process the subdirectory
        if let Ok(subdir_lines) = process_directory(src_path, &subdir, 0) {
            lines.extend(subdir_lines);
        }
        lines.push(String::new());
    }
    
    Ok(lines.join("\n"))
}

/// Capitalize first letter of a string (simple title case)
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_uppercase().to_string();
            result.push_str(&chars.as_str());
            result
        }
    }
}

fn main() -> io::Result<()> {
    // Get the src directory path
    let src_path = PathBuf::from("src");
    
    if !src_path.exists() {
        eprintln!("Error: {:?} does not exist", src_path);
        std::process::exit(1);
    }
    
    // Generate SUMMARY content
    let summary_content = generate_summary(&src_path)?;
    
    // Write to SUMMARY.md
    let summary_path = src_path.join("SUMMARY.md");
    let mut file = fs::File::create(&summary_path)?;
    file.write_all(summary_content.as_bytes())?;
    
    println!("Successfully updated {:?}", summary_path);
    
    Ok(())
}
