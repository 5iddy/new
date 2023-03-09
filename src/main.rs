use clap::Parser;
use new::args::Args;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use walkdir::WalkDir;

fn main() {
    let path = dotenvy::dotenv().unwrap();
    pretty_env_logger::init();

    log::debug!("path: {path:#?}");

    let args = Args::parse();
    log::debug!("args: {:#?}", args);

    let default_templates_dir = env::var("HOME")
        .map(PathBuf::from)
        .map(|home_dir| home_dir.join("Templates"))
        .unwrap();

    let templates_dir = args.templates_dir.map_or(default_templates_dir, PathBuf::from);
    log::debug!("template_dir: {templates_dir:#?}");

    if !templates_dir.is_dir(){
        log::error!("Templates Directory doesn't exist: {templates_dir:#?}.");
        exit(1);
    }

    for entry in WalkDir::new(templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let filename;
        if path.is_file() && path.file_name().is_some() {
            filename = path.file_name().unwrap().to_str().unwrap().to_string();
        } else {
            continue;
        };
        if args.files.contains(&filename) {
            log::info!("filename: {:#?}", filename);
        }
    }
}
