# SZ
Prints the size of a file in human readable format blazingly fast

Go version -> https://github.com/elliot40404/sz

## Why?
Because I am on windows and it is stupid


## Installation
```console
$ cargo install sz
```

```console
$ git clone
$ cd szr
$ cargo build --release
$ cargo install --path .
```

## Usage

```console
$ szr
szr v0.1.0 by Elliot40404
Prints file sizes in bytes, kilobytes, megabytes, and gigabytes
Usage: sz <file> <file> <file> ...
```

```console
$ szr .\README.md
677 B  0.66 KB  0.00 MB  0.00 GB  ".\\README.md"
```

```console
$ szr README.md sz.jsonl
677 B  0.66 KB  0.00 MB  0.00 GB  ".\\README.md"
159 B  0.16 KB  0.00 MB  0.00 GB  ".\\Cargo.toml"
```

## License
MIT