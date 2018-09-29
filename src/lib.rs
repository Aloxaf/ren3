extern crate regex;
extern crate failure;

use failure::Error;
use regex::RegexBuilder;
use std::fs;
use std::fs::DirEntry;

pub struct FilterArgs {
    pub dir_only: bool,
    pub file_only: bool,
    pub hidden_file: bool,
    pub recursive: bool,
}

pub struct RenameArgs {
    pub case_insensitive: bool,
    pub apply: bool,
    pub brief: bool,
}

pub fn list_files(dir: &str, args: &FilterArgs) -> Result<Vec<DirEntry>, Error>  {
    let paths = fs::read_dir(dir)?;
    let mut ret = Vec::new();
    for path in paths {
        let path = path.unwrap();

        let file = path.path();
        if !args.hidden_file && file.file_name().unwrap().to_str().unwrap().starts_with(".") {
            continue
        } else if args.dir_only && file.is_dir() {
            ret.push(path);
        } else if args.file_only && file.is_file() {
            ret.push(path);
        } else {
            ret.push(path);
        }

        if args.recursive && file.is_dir() {
            ret.extend(list_files(file.to_str().unwrap(), args).unwrap());
        }

    }
    Ok(ret)
}


pub fn rename(pattern: &str, repl: &str, files: Vec<DirEntry>, args: &RenameArgs) -> Result<(), Error> {

    let re = if args.case_insensitive {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()?
    } else {
        RegexBuilder::new(pattern).build()?
    };

    for path in files {
        let old_path = path.path();
        let mut new_path = old_path.clone();

        if !re.is_match(old_path.file_name().unwrap().to_str().unwrap()) {
            continue;
        }

        let new_file_name = re.replace(old_path.file_name().unwrap().to_str().unwrap(), repl);
        new_path.set_file_name(&new_file_name.to_string());

        let old_name = old_path.to_str().unwrap();
        let new_name = new_path.to_str().unwrap();

        if args.apply {
            match fs::rename(old_name, new_name) {
                Err(e) => {
                    eprintln!("Failed to rename {}: {}", old_name, e);
                    continue;
                },
                _ => (),
            }
        }

        if args.brief {
            println!("{}\t-> {}",
                     old_path.file_name().unwrap().to_str().unwrap(),
                     new_path.file_name().unwrap().to_str().unwrap());
        } else {
            println!("{}\t-> {}", old_name, new_name);
        }
    }
    Ok(())
}