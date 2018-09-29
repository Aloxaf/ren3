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


extern crate regex;

use regex::RegexBuilder;
use std::fs;
use std::path::PathBuf;
use std::process;

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

pub fn list_files(dir: &str, args: &FilterArgs) -> Vec<PathBuf>  {
    let mut ret = Vec::new();
    let paths = fs::read_dir(dir).unwrap_or_else(|e| {
        eprintln!("Unable to open directory '{}': {}", dir, e);
        process::exit(1);
    });

    for path in paths {
        let path = path.unwrap();
        let path = path.path();

        if (!args.hidden_file && path.file_name().unwrap().to_str().unwrap().starts_with("."))
            || (args.dir_only && !path.is_dir())
            || (args.file_only && !path.is_file()) {
            continue
        }

        ret.push(path.clone());

        if args.recursive && path.is_dir() {
            ret.extend(list_files(path.to_str().unwrap(), args));
        }
    }

    ret
}


pub fn rename(pattern: &str, repl: &str, files: Vec<PathBuf>, args: &RenameArgs) {

    let re = if args.case_insensitive {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
    } else {
        RegexBuilder::new(pattern).build()
    };

    let re = re.unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    for old_path in files {
        let old_file_name = old_path.file_name().unwrap().to_str().unwrap();

        if !re.is_match(old_file_name) {
            continue;
        }

        let new_file_name = re.replace(old_file_name, repl);

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
