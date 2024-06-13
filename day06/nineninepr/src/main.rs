use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

struct MultiplicationTable {
    max_factor: usize,
    align_width: usize,
}

impl MultiplicationTable {
    fn new(max_factor: usize, align_width: usize) -> Self {
        MultiplicationTable {
            max_factor,
            align_width,
        }
    }

    fn generate(&self) -> String {
        let mut table = String::new();
        for i in 1..=self.max_factor {
            for j in 1..=i {
                table += &format!("{1:>0$}x{2:>0$}={3:>0$}\t", self.align_width, j, i, i * j);
            }
            table.push('\n');
        }
        table
    }

    fn write_to_file(&self, file_path: &Path) -> io::Result<()> {
        let table = self.generate();
        let mut file = File::create(file_path)?;
        file.write_all(table.as_bytes())?;
        Ok(())
    }
}

impl fmt::Display for MultiplicationTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generate())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_factor = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(9);
    let align_width = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2);

    let table = MultiplicationTable::new(max_factor, align_width);
    println!("{}", table);

    if let Some(file_path) = args.get(3) {
        let path = Path::new(file_path);
        if let Err(e) = table.write_to_file(path) {
            eprintln!("Failed to write to file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication_table_size() {
        let table = MultiplicationTable::new(9, 2);
        let output = table.generate();
        // Check the number of lines which corresponds to the number of rows (9 for 9x9 table)
        assert_eq!(output.trim().matches('\n').count(), 8);
    }

    #[test]
    fn test_multiplication_table_content() {
        let table = MultiplicationTable::new(9, 2);
        let output = table.generate();
        // Check specific content correctness
        assert!(output.contains(" 1x 1= 1\t"));
        assert!(output.contains(" 2x 2= 4\t"));
        assert!(output.contains(" 9x 9=81\t"));
    }

    #[test]
    fn test_output_alignment() {
        let table = MultiplicationTable::new(9, 3);
        let output = table.generate();
        // Check if alignment is correct by examining specific entries
        assert!(output.contains("  1x  1=  1\t"));
        assert!(output.contains("  9x  9= 81\t"));
    }
}
