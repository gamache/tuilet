# tuilet

![Crates.io Version](https://img.shields.io/crates/v/tuilet)

A TUI for [toilet](https://github.com/cacalabs/toilet), the premier
command-line ANSI text generator.

Tuilet makes it easy to experiment with fonts and command line options.

Requires a Unix-like OS where `toilet` is installed.

![A screenshot of Tuilet](/screenshots/0.3.0.png?raw=true)

## Usage

Tab selects the active textarea.
Select fonts with the Up and Down arrows.
Output and command line can be copied by pressing Enter.

Exit with Ctrl-C. At program exit, tuilet prints the most
recent toilet command line to the terminal.

```
Usage: tuilet [OPTIONS]

Options:
  -D, --fontdir <PATH>  Add fonts from the given directory
  -X, --toilet <PATH>   Set path to `toilet` executable
  -h, --help            Print help
  -V, --version         Print version
```

## TODO

- Better font selector
- Internal font pack
- Config file
- ???

## Bugs

Oh yeah definitely

## Contributing

PRs and Issues welcome.

Let a smile be your Contributor License Agreement

## Authorship and License

This code was written in 2024 by [Pete Gamache](mailto:pete@gamache.org).

Tuilet is released under the [BSD License](LICENSE.txt).
