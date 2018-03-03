extern crate regex;

use regex::Regex;
use std::io::BufRead;

pub fn count_the(reader: &mut BufRead) -> usize {
    let re = Regex::new("[^a-z0-9]").unwrap();

    reader
        .split(' ' as u8)
        .map(|s| -> String { String::from_utf8(s.unwrap()).unwrap().to_lowercase() })
        .map(|s| re.replace_all(s.as_str(), "").to_string())
        .filter(|s| s == "the")
        .count()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use count_the;

    #[test]
    fn it_the_single_the() {
        let input_data = "hello to the world";
        let mut reader = BufReader::new(input_data.as_bytes());

        assert_eq!(count_the(&mut reader), 1);
    }

    #[test]
    fn it_counts_the_multiple_the() {
        let input_data = "the quick brown fox jumps over the lazy dog";
        let mut reader = BufReader::new(input_data.as_bytes());

        assert_eq!(count_the(&mut reader), 2);
    }

    #[test]
    fn it_counts_ignores_capitalisation() {
        let input_data = "The quick brown fox jumps over the lazy dog";
        let mut reader = BufReader::new(input_data.as_bytes());

        assert_eq!(count_the(&mut reader), 2);
    }

    #[test]
    fn it_ignores_punctuation() {
        let input_data = "Their name is The.";
        let mut reader = BufReader::new(input_data.as_bytes());

        assert_eq!(count_the(&mut reader), 1);
    }

    #[test]
    fn it_null_characters_at_the_end_of_string_are_stripped() {
        let input_data = "Their name is The\0";
        let mut reader = BufReader::new(input_data.as_bytes());

        assert_eq!(count_the(&mut reader), 1);
    }
}
