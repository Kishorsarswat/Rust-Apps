use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let output = input.graphemes(true).rev().collect::<String>();
    output
}
