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
use clap::{load_yaml, App};
use colored::*;
use ren3::{list_and_rename_files, Args, SedRegex};

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let option_exist = |s: &str| -> bool { matches.occurrences_of(s) == 1 };

    let expression = matches.value_of("expression").unwrap();
    let dirs = match matches.values_of("dir") {
        Some(values) => values.collect::<Vec<_>>(),
        None => vec!["."],
    };

    let args = Args {
        dir_only: option_exist("dir-only"),
        file_only: option_exist("file-only"),
        recursive: option_exist("recursive"),
        apply: option_exist("force"),
        brief: option_exist("brief"),
    };

    let re = SedRegex::new(expression);

    for dir in dirs {
        list_and_rename_files(&re, dir, &args);
    }

    if !args.apply {
        println!(
            "{}",
            "\n\nTHIS IS DEMO MODE.\nUSE '-f' OPTION TO APPLY CHANGES."
                .bright_yellow()
                .bold()
        );
    }
}
