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

    let mut pattern = matches.value_of("pattern").unwrap().to_string();
    let repl = matches.value_of("repl").unwrap();
    let dir = matches.value_of("dir").unwrap_or(".");

    let filter_args = FilterArgs {
        dir_only: option_exist("dir-only"),
        file_only: option_exist("file-only"),
        hidden_file: option_exist("all"),
        recursive: option_exist("recursive"),
    };

    let rename_args = RenameArgs {
        case_insensitive: option_exist("case-insensitive"),
        apply: option_exist("force"),
        brief: option_exist("brief"),
    };

    if option_exist("strict-mode") {
        pattern = format!("^{}$", pattern);
    }

    let files = list_files(dir, &filter_args);
    rename(&pattern, repl, files, &rename_args);
}
