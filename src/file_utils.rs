use std::io;
use std::io::Write;
use std::fs::File;
use crate::sample_cases::SampleCases;


// Create file of sample cases for each element of `SampleCases` struct.
pub fn create_test_files(sc: SampleCases) -> Result<(), io::Error> {
    for (i, input_example) in sc.input.iter().enumerate() {
        let file_name = format!("input{}.txt", i + 1);
        create_test_file(input_example, file_name).expect("Failed to create file.");
    }

    for (i, output_example) in sc.output.iter().enumerate() {
        let file_name = format!("output{}.txt", i + 1);
        create_test_file(output_example, file_name).expect("Failed to create file.");
    }
    Ok(())
}

// Write `file_content` on a file.
fn create_test_file(file_content: &String, file_name: String) -> Result<(), io::Error> {
    let file = File::create(file_name)?;
    let mut writer = io::BufWriter::new(file);
    write!(writer, "{}", file_content).unwrap();
    Ok(())
}
