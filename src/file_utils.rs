use std::env;
use std::io;
use std::io::Write;
use std::fs::{ self, File, DirBuilder };
use crate::sample_cases::SampleCases;
use std::path::Path;


// Create file of sample cases for each element of `SampleCases` struct.
pub fn create_test_files(sc: SampleCases) -> Result<(), io::Error> {
    create_directory("input")?;
    create_directory("output")?;

    for (i, input_example) in sc.input.iter().enumerate() {
        let file_name = format!(r"input\input{}.txt", i + 1);
        create_test_file(input_example, file_name).expect("Failed to create file.");
    }

    for (i, output_example) in sc.output.iter().enumerate() {
        let file_name = format!(r"output\output{}.txt", i + 1);
        create_test_file(output_example, file_name).expect("Failed to create file.");
    }
    Ok(())
}

// Create directory named `dir_name` under the current directory.
fn create_directory(dir_name: &str) -> io::Result<()> {
    let mut dir = env::current_dir()?;
    dir.push(dir_name);
    if dir.is_dir() {
        Ok(())
    }
    else {
        DirBuilder::new().create(dir)
    }
}

// Write `file_content` on a file.
fn create_test_file(file_content: &String, file_name: String) -> Result<(), io::Error> {
    let file = File::create(file_name)?;
    let mut writer = io::BufWriter::new(file);
    write!(writer, "{}", file_content).unwrap();
    Ok(())
}
