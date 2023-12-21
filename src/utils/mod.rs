use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use crate::{error::Result, iterator::ext::merge, models::lines::Line};

pub fn filter_merge_and_write_lines<T, F, M>(
    input_files: Vec<T>,
    output_file_path: T,
    filter_predicate: F,
    merge_predicate: M,
) -> Result<()>
where
    T: AsRef<str>,
    F: Fn(&Line) -> bool,
    M: Fn(&Line, &Line) -> bool,
{
    // First, create the iterators for each of the input files using the Line struct, and apply the filters.
    let mapping_fn = |i: (usize, io::Result<String>)| {
        let idx = i.0;
        let line = i.1.expect("Unable to read line");

        Line::new(idx + 1, line)
    };

    let mut iterators = Vec::new();
    for input_file in &input_files {
        let file = File::open(input_file.as_ref())?;
        let reader = BufReader::new(file);
        let iter = reader
            .lines()
            .enumerate()
            .map(mapping_fn)
            .filter(&filter_predicate);
        iterators.push(iter);
    }

    // Then, start merging each pair of the iterators. For N files we will use N - 1 merge operations.
    let mut merged_iter: Box<dyn Iterator<Item = Line>> =
        Box::new(iterators.pop().expect("No iterators found"));
    while let Some(other_iter) = iterators.pop() {
        let other_iter: Box<dyn Iterator<Item = Line>> = Box::new(other_iter);
        merged_iter = Box::new(merge(merged_iter, other_iter, &merge_predicate));
    }

    // Finally, write the merged iterator to the output file.
    let mut file = File::create(output_file_path.as_ref())?;
    for line in merged_iter {
        writeln!(file, "{}", line.line_contents())?;
    }

    Ok(())
}

pub fn filter_and_write_lines<T, F>(
    input_file_path: T,
    filter_predicate: F,
    output_file_path: T,
) -> Result<()>
where
    T: AsRef<str>,
    F: Fn(&Line) -> bool,
{
    // First, create the iterator for the input file using the Line struct, and apply the filter.
    let mapping_fn = |i: (usize, io::Result<String>)| {
        let idx = i.0;
        let line = i.1.expect("Unable to read line");

        Line::new(idx + 1, line)
    };

    let file = File::open(input_file_path.as_ref())?;
    let reader = BufReader::new(file);
    let iter = reader
        .lines()
        .enumerate()
        .map(mapping_fn)
        .filter(&filter_predicate);

    // Then, write the iterator to the output file.
    let mut file = File::create(output_file_path.as_ref())?;
    for line in iter {
        writeln!(file, "{}", line.line_contents())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_merge_and_write_lines() {
        let input_files = vec![
            "./test_data/test_1.log",
            "./test_data/test_2.log",
            "./test_data/test_3.log",
        ];

        let output_file = "./test_data/merged_1_2_3.log";

        let filter_fn = |l: &Line| l.line_number() % 2 == 0;

        let merge_fn = |line1: &Line, line2: &Line| {
            let line1_number = line1.line_contents().chars().next().unwrap() as usize;
            let line2_number = line2.line_contents().chars().next().unwrap() as usize;

            line1_number < line2_number
        };

        filter_merge_and_write_lines(input_files, output_file, filter_fn, merge_fn)
            .expect("Unable to merge files");

        // Now check the output file.
        let file = File::open(output_file).expect("Unable to open file");
        let reader = BufReader::new(file);
        let mut iter = reader.lines();

        assert_eq!(iter.next().unwrap().unwrap(), "3: three".to_string());
        assert_eq!(iter.next().unwrap().unwrap(), "5: five".to_string());
        assert_eq!(iter.next().unwrap().unwrap(), "6: six".to_string());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_filter_and_write_lines() {
        let input_file = "./test_data/test_1.log";
        let output_file = "./test_data/filtered_1.log";

        let filter_fn = |l: &Line| l.line_number() % 2 == 0;

        filter_and_write_lines(input_file, filter_fn, output_file).expect("Unable to filter file");

        // Now check the output file.
        let file = File::open(output_file).expect("Unable to open file");
        let reader = BufReader::new(file);
        let mut iter = reader.lines();

        assert_eq!(iter.next().unwrap().unwrap(), "5: five".to_string());
        assert!(iter.next().is_none());
    }
}
