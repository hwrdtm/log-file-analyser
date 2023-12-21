use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use log_file_analyser::{
    iterator::ext::{merge, Printer},
    models::lines::LineItem,
};

fn main() {
    let file1 = File::open("./test_1.log").expect("Unable to open file");
    let file2 = File::open("./test_2.log").expect("Unable to open file");

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    // Create an iterator through each line using the LineItem struct.
    let mapping_fn = |i: (usize, Result<String, Error>)| {
        let idx = i.0;
        let line = i.1.expect("Unable to read line");

        LineItem::new(idx + 1, line)
    };

    let lines1 = reader1.lines().enumerate().map(mapping_fn);
    let lines2 = reader2.lines().enumerate().map(mapping_fn);

    // Merge the iterators
    let merged = merge(lines1, lines2, |line1, line2| {
        let line1_number = line1.line_contents().chars().next().unwrap() as usize;
        let line2_number = line2.line_contents().chars().next().unwrap() as usize;

        line1_number < line2_number
    });

    // Write the merged iterator to stdout using the Printer iterator.
    let iter = Printer::new(merged);
    iter.collect::<Vec<_>>();
}
