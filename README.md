# tuilet

A TUI for [toilet](https://github.com/cacalabs/toilet), the premier
command-line ANSI text generator.

Tuilet makes it easy to experiment with fonts and command line
options, showing output in real time and displaying the Toilet
command for easy copy-pasting.

Requires a Unix-like OS where `toilet` is installed.

![A screenshot of Tuilet](/screenshots/0.2.0.png?raw=true)

## Usage

Run tuilet at the command line. You can tab between the Input, Font, and
Flags fields. Exit with Ctrl-C. At program exit, tuilet prints the most
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
- Clipboard support
- ???

## Bugs

Oh yeah definitely

## Contributing

PRs and Issues welcome.

Let a smile be your Contributor License Agreement

## Authorship and License

This code was written in 2024 by [Pete Gamache](mailto:pete@gamache.org).

Tuilet is released under the [BSD License](LICENSE.txt).
