use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_csv_rows(
    file_name: &String,
    delimiter: &char,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let handler = File::open(file_name)?;
    let mut buffer = BufReader::new(handler);
    let mut content = String::new();
    buffer.read_to_string(&mut content)?;

    Ok(content
        .lines()
        .map(|line| {
            line.split(*delimiter)
                .map(|item| item.to_string())
                .collect()
        })
        .collect())
}
