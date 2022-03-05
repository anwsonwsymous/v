extern crate crossterm;
extern crate tui;

use argh::FromArgs;

mod ui;
mod app;
mod terminal;
mod file_reader;


/// View csv files in table view (not only csv files)
#[derive(FromArgs)]
struct Cli {
    /// file path
    #[argh(positional)]
    file_name: String,

    /// separator character
    #[argh(option, short = 'd', default = "','")]
    descriptor: char,

    /// show headers in table
    #[argh(switch, short = 'h')]
    headers: bool,
}

fn main() {
    // Parse cli arguments
    let cli: Cli = argh::from_env();

    // Read file content
    // TODO: Optimize to return file size and iterator
    let rows = file_reader::read_csv_rows(&cli.file_name, &cli.descriptor).unwrap();
    terminal::run(rows, cli.file_name, cli.headers);
}