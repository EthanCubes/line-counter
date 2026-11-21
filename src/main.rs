fn main() {
    let mut files: Vec<String> = std::vec::Vec::new();

    files.push("".to_string());

    let mut line_count = 0;

    for file in &files {
        // count the amount of \n characters inside of the string
        let lines = file.matches("\n").count() + 1;
        line_count += lines;
    }

    println!("{}", line_count);
}
