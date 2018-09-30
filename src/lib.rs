// Copyright (C) 2018 by Aloxaf
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.


extern crate sedregex;

use std::fs;
use std::path::PathBuf;
use std::process;

use sedregex::{split_for_replace, ErrorKind};

pub struct FilterArgs {
    pub dir_only: bool,
    pub file_only: bool,
    pub recursive: bool,
}

pub struct RenameArgs {
    pub apply: bool,
    pub brief: bool,
}

pub fn list_files(dir: &str, args: &FilterArgs) -> Vec<PathBuf>  {
    let mut ret = Vec::new();
    let paths = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Unable to open directory '{}': {}", dir, e);
            return vec![];
        }
    };

    for path in paths {
        let path = path.unwrap();
        let path = path.path();

        if args.recursive && path.is_dir() {
            ret.extend(list_files(path.to_str().unwrap(), args));
        }

        if (args.dir_only && !path.is_dir()) || (args.file_only && !path.is_file()) {
            continue
        }

        ret.push(path.clone());
    }

    ret
}


pub fn rename(expression: &str, files: Vec<PathBuf>, args: &RenameArgs) {

    let replace_data = split_for_replace(expression).unwrap_or_else(|e| {
        eprintln!("regex split error: {:?}", e);
        process::exit(1);
    });

    let re = replace_data.build_regex().unwrap_or_else(|e| {
        if let ErrorKind::RegexError(e) = e {
            eprintln!("{}", e);
        }
        process::exit(1);
    });

    for old_path in files {
        let old_file_name = old_path.file_name().unwrap().to_str().unwrap();

        if !re.is_match(old_file_name) {
            continue;
        }

        let new_file_name = if replace_data.flags.is_global() {
            re.replace_all(old_file_name, replace_data.with.as_ref())
        } else {
            re.replace(old_file_name, replace_data.with.as_ref())
        };

        let mut new_path = old_path.clone();
        new_path.set_file_name(&new_file_name.to_string());

        let old_name = old_path.to_str().unwrap();
        let new_name = new_path.to_str().unwrap();

        if args.apply {
            if let Err(e) = fs::rename(old_name, new_name) {
                eprintln!("Failed to rename '{}': {}", old_name, e);
                continue;
            }
        }

        if args.brief {
            println!("{}\t-> {}",
                     old_file_name,
                     new_file_name);
        } else {
            println!("{}\t-> {}", old_name, new_name);
        }
    }
}
