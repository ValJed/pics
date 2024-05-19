use clap::Parser;
use exif::{In, Tag};
use std::fs;

mod args;
use args::Cli;

const IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];
const CHARS_TO_REMOVE: [char; 2] = ['-', ':'];

fn main() {
    let cli = Cli::parse();
    // let is_recursive = cli.recursive;
    let path = cli.path;

    let files_res = fs::read_dir(path);

    if files_res.is_err() {
        eprintln!("Error reading directory: {}", files_res.unwrap_err());
        return;
    }

    let files = files_res.unwrap();

    for entry_res in files {
        if entry_res.is_err() {
            eprintln!("Error reading entry: {}", entry_res.unwrap_err());
            continue;
        }
        let entry = entry_res.unwrap();
        let file_type_res = entry.file_type();
        if file_type_res.is_err() {
            continue;
        }

        let file_type = file_type_res.unwrap();
        if !file_type.is_file() {
            continue;
        }

        let entry_path = entry.path();
        let file_name_res = entry.file_name().into_string();

        if file_name_res.is_err() {
            eprintln!("No file name found for: {}", entry_path.display());
            continue;
        }
        let file_name = file_name_res.unwrap();
        let extension_res = entry_path.extension();
        if extension_res.is_none() {
            continue;
        }
        let extension_str = extension_res.unwrap().to_str();
        if extension_str.is_none() {
            continue;
        }

        let ext = extension_str.unwrap().to_lowercase();
        if !IMAGE_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        let file = fs::File::open(entry_path.clone());
        if file.is_err() {
            eprintln!("Error reading file: {}", file.unwrap_err());
            continue;
        }

        let mut bufreader = std::io::BufReader::new(file.unwrap());
        let exifreader = exif::Reader::new();
        let exif_res = exifreader.read_from_container(&mut bufreader);
        if exif_res.is_err() {
            eprintln!("Error reading exif for image: {}", file_name);
            continue;
        }
        let exif = exif_res.unwrap();
        let date_time_res = exif.get_field(Tag::DateTime, In::PRIMARY);
        if date_time_res.is_none() {
            continue;
        }

        let date_time = date_time_res.unwrap().display_value().to_string();
        let name = format_name(date_time, ext);
        let file_path_opt = entry_path.to_str();
        if file_path_opt.is_none() {
            continue;
        }
        let file_path = file_path_opt.unwrap();
        let new_path = file_path.replace(file_name.as_str(), name.as_str());

        match fs::rename(file_path, new_path) {
            Ok(_) => {
                println!("Renamed {} -> {}", file_name, name)
            }
            Err(err) => {
                eprintln!("Error when renaming: {}", err)
            }
        }
    }
}

fn format_name(name: String, ext: String) -> String {
    let name = name
        .chars()
        .filter(|char| !CHARS_TO_REMOVE.contains(char))
        .collect::<String>()
        .replace(" ", "_");

    format!("{}.{}", name, ext)
}
