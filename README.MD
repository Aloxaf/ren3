# ren3 (WIP)

Do file renaming using regular expressions.

Inspired by [ren2](http://www.lomont.org/Software/)

[![Linux build status](https://travis-ci.org/Aloxaf/ren3.svg)](https://travis-ci.org/Aloxaf/ren3)

## Install

```bash
cargo install --git https://github.com/Aloxaf/ren3
```

## Usage
```text
USAGE:
    ren3 [FLAGS] <expression> [dir]...

FLAGS:
    -b, --brief             only display base name
    -D, --directory-only    only rename directories
    -F, --file-only         only rename files
    -f, --force             apply changes
    -h, --help              Prints help information
    -r, --recursive         rename subdirectories recursively
    -V, --version           Prints version information

ARGS:
    <expression>    sed-like expression
    <dir>...        directories to rename
```

## Examples

```bash
> ls
foo-1.jpg foo2.jpg

> ren3 's/.*(\d+\.jpg)/bar-$1'
./foo2.jpg      -> ./bar-2.jpg
./foo-1.jpg     -> ./bar-1.jpg

> ls
bar-1.jpg  bar-2.jpg
```
