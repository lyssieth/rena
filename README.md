# rena

![crates.io](https://img.shields.io/crates/v/rena)

A hopefully-simple bulk-renaming utility.

## Requirements

- clap
- color-eyre
- paris
- strfmt
- regex

## Usage

The most basic usage is simply `rena <folder>`, which runs against a folder, renaming everything with the pattern of `item_{:10>number}`, where number is the item's number when being read.  
The padding amount can be adjusted with `--padding <number>`, and the direction with `--padding-direction <direction>`, and the prefix with `--prefix <prefix>`.

It can also run in regex mode, by default as a filter if only `--match <regex>` is used. If `--match-rename <pattern>` is also used, it renames everything based on a pattern. The exact syntax is described in `--help`.

See `-h` or `--help` for all flags.

### Examples

#### Most Basic

Let's say there's a directory named `images` with the following structure:

```md
image.jpg
image3.jpg
12746uju21.jpg
17f29a002.jpg
```

After running `rena images/`, it will result in:

```md
item_0000000000.jpg
item_0000000001.jpg
item_0000000002.jpg
item_0000000003.jpg
```
