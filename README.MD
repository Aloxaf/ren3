# ren3 (WIP)

Do file renaming using regular expressions.

Inspired by [ren2](http://www.lomont.org/Software/)

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
