# rena

![crates.io](https://img.shields.io/crates/v/rena)

A hopefully-simple bulk-renaming utility.

Its capabilities currently include dry-run, regex and (simply?) tweaked output.

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

Currently only has the most basic execution mode covered, I hope to add more.

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

#### Regex Filtering

Let's say there's a directory named `images` with the following structure:

```md
image.jpg
image3.mp4
12746uju21.jpg
17f29a002.jpg
17f2121wss.png
ffe_image_breaker.webm
potential_effort.jpg
```

After running `rena --match "\.[jpgn]+" images/`, it will result in:

```md
ffe_image_breaker.webm
image3.mp4
item_0000000000.jpg
item_0000000001.jpg
item_0000000002.png
item_0000000003.jpg
item_0000000004.jpg
```

#### Regex Usage

Let's say there's a directory named `Show` with the following structure:

```md
Show.S01E01.1080p.mkv
Show.S01E02.1080p.mkv
Show.S01E03.1080p.mkv
Show.S02E01.1080p.mkv
Show.S02E02.1080p.mkv
Show.S02E03.1080p.mkv
```

After running `rena --match "Show\.S(\d+)E(\d+)\.1080p\.mkv" --match-rename "Show S${1} E${2} (1080p).mkv" Show/`, it will result in:  
<sub>Note: most shells will require escaping the $-sign</sub>

```md
Show S01 E02 (1080p).mkv
Show S01 E03 (1080p).mkv
Show S02 E01 (1080p).mkv
Show S01 E01 (1080p).mkv
Show S02 E02 (1080p).mkv
Show S02 E03 (1080p).mkv
```
