use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn read_csv_rows(file_name: &String, descriptor: &char) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut handler = File::open(file_name)?;
    let mut content = String::new();
    handler.read_to_string(&mut content)?;

    Ok(content.lines().map(|line| {
        line.split(*descriptor)
            .map(|item| item.to_string())
            .collect()
    }).collect())
}