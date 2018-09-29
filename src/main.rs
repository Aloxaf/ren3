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

    let files = list_files(dir, &filter_args).unwrap();
    rename(&pattern, repl, files, &rename_args).unwrap();
}
