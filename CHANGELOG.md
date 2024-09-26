# Changelog

## 0.3.1 (2024-09-26)

Fixed handling of non-functioning clipboard.

## 0.3.0 (2024-09-26)

Added copy-to-clipboard support for Output and Command Line fields.

## 0.2.0 (2024-09-20)

This release adds:

- ANSI-colored output support (e.g., `--gay` and `--metal`) via ansi-to-tui
- command line argument handling via clap
  - `-D/--fontdir` argument adds fonts from the given directory
  - `-X/--toilet` argument sets the `toilet` executable path

## 0.1.0 (2024-09-13)

Initial release on Friday the 13th. Basic font selector, input and flags
textboxes, output and command displays. No support for colored output.
