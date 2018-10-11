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

extern crate colored;
extern crate regex;
extern crate sedregex;

use colored::*;
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsString;
use std::io;
use std::process;

use sedregex::{split_for_replace, ErrorKind};

#[derive(Debug)]
enum Error {
    UnknownFileName(OsString),
    RenameError(String, String, io::Error),
}

pub struct Args {
    pub dir_only: bool,
    pub file_only: bool,
    pub recursive: bool,
    pub apply: bool,
    pub brief: bool,
}

pub struct SedRegex<'a> {
    re: regex::Regex,
    rep: Cow<'a, str>,
    global: bool,
}

impl<'a> SedRegex<'a> {
    pub fn new(expression: &str) -> SedRegex {
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
        SedRegex {
            re,
            rep: replace_data.with,
            global: replace_data.flags.is_global()
        }
    }

    #[inline]
    pub fn replace<'t>(&self, text: &'t str) -> Cow<'t, str> {
        if self.global {
            self.re.replace_all(text, self.rep.as_ref())
        } else {
            self.re.replace(text, self.rep.as_ref())
        }
    }

    #[inline]
    pub fn is_match(&self, text: &str) -> bool {
        self.re.is_match(text)
    }
}

pub fn list_and_rename_files(re: &SedRegex, dir: &str, args: &Args) {

    let paths = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Unable to open directory '{}': {}", dir, e);
            return;
        }
    };

    for path in paths {
        let path = path.unwrap();
        let path = path.path();

        if args.recursive && path.is_dir() {
            list_and_rename_files(re, path.to_str().unwrap(), args);
        }

        if (args.dir_only && !path.is_dir()) || (args.file_only && !path.is_file()) {
            continue
        }

        rename_file(&re, &path, args).unwrap_or_else(|e| {
            match e {
                Error::UnknownFileName(filename) => eprintln!("{}", format!("[ERROR] Unknown file name {:?}", filename).bright_red()),
                Error::RenameError(old, new, err) => eprintln!("{}", format!("[ERROR] Failed to rename {} -> {}: {}", old, new, err).bright_red()),
            }
        });
    }
}

#[inline]
fn rename_file(re: &SedRegex, file: &PathBuf, args: &Args) -> Result<(), Error> {
    let old_path = file;
    let old_file_name = match old_path.file_name().unwrap().to_str() {
        Some(value) => value,
        None => return Err(Error::UnknownFileName(old_path.file_name().unwrap().to_os_string())),
    };

    if !re.is_match(old_file_name) {
        return Ok(());
    }

    let new_file_name = re.replace(old_file_name);

    let mut new_path = old_path.clone();
    new_path.set_file_name(&new_file_name.to_string());

    let old_full_name = old_path.to_str().unwrap();
    let new_full_name = new_path.to_str().unwrap();

    if args.apply {
        if let Err(e) = fs::rename(old_full_name, new_full_name) {
            return Err(Error::RenameError(old_full_name.to_string(), new_full_name.to_string(), e));
        }
    }

    if args.brief {
        println!("{}", format!("[OK] {}\t-> {}", old_file_name, new_file_name).bright_green());
    } else {
        println!("{}", format!("[OK] {}\t-> {}", old_full_name, new_full_name).bright_green());
    }
    Ok(())
}
