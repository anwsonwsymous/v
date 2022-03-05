# What the **** is this

File content viewer(UTF-8) implemented with `tui-rs`.
Designed to open (BIG) `csv` files with table view.

## How to install ?

Build:
```
cargo build --release
```
## Usage
```
Usage: v.exe <file_name> [-d <descriptor>] [-h]

View csv files in table view (not only csv files)

Positional Arguments:
  file_name         file path

Options:
  -d, --descriptor  separator character
  -h, --headers     show headers in table
  --help            display usage information
```

## Keys

```
controls    - ←                             unselect
            - ↓↑ | scroll                   next, previous
            - <Shift> + ↓↑ | scroll         Go fast
            - <Shift+Ctrl> + ↓↑ | scroll    Go more faster
            - PageUp | PageDown             Go super fast
```