# What the **** is this

File content viewer(UTF-8) implemented in `tui-rs`.
Designed to open (BIG) `csv` files with table view.

## How to install ?

Build:
```
cargo build --release
```
## Usage
```
Usage: v.exe <file_name> [-d <delimiter>] [-h]

View csv files in table view (not only csv files)

Positional Arguments:
  file_name         file path

Options:
  -d, --delimiter   separator character
  -h, --headers     show headers in table
  --help            display usage information
```

## Keys

```
controls    - ← | Esc                       unselect
            - ↓↑ | scroll                   next, previous
            - <Shift> + ↓↑ | scroll         Go fast
            - <Shift+Ctrl> + ↓↑ | scroll    Go faster
            - PageUp | PageDown             Go super fast
```