use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_vec(filepath: &str) -> io::Result<Vec<String>> {
    let lines = read_lines(filepath)?;
    Ok(lines.flatten().collect())
}