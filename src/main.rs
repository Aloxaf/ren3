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

#[macro_use] extern crate clap;
extern crate ren3;

use clap::App;
use ren3::{list_files, rename, FilterArgs, RenameArgs};


fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let option_exist = |s: &str| -> bool {
        matches.occurrences_of(s) == 1
    };

    let expression = matches.value_of("expression").unwrap();
    let dirs = match matches.values_of("dir") {
        Some(values) => values.collect::<Vec<_>>(),
        None => vec!["."],
    };

    let filter_args = FilterArgs {
        dir_only: option_exist("dir-only"),
        file_only: option_exist("file-only"),
        recursive: option_exist("recursive"),
    };

    let rename_args = RenameArgs {
        apply: option_exist("force"),
        brief: option_exist("brief"),
    };

    for dir in dirs {
        let files = list_files(dir, &filter_args);
        rename(expression, files, &rename_args);
    }

    if !rename_args.apply {
        println!("\n\nTHIS IS DEMO MODE.\nUSE '-f' OPTION TO APPLY CHANGES.");
    }
}
