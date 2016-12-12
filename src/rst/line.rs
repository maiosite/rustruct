// MIT License
//
// Copyright (c) 2016 Fuzhou Chen
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject
// to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::error::Error;

static SECTION_RECOMMENDED_ADORNMENT: &'static str = "=-`:.'\"~^_*+#";
static SECTION_ACCEPTABLE_ADORNMENT: &'static str = "!$%&(),/;<>?@[\\]{|}";
static ITEMIZED_BULLET: &'static str = "*+-•‣⁃";

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Line {
    BlankLine,

    Adornment {
        adornment_char: char,
        adornment_length: usize,
        err: Option<Error>,
    },

    Text {
        indention_count: usize,
        itemized_like: bool,
    },

    EndOfFile,
}

fn detect_blank_line(line: &str) -> Option<Line> {
    if line.trim().len() == 0 {
        Some(Line::BlankLine)
    } else {
        None
    }
}

fn detect_recommended_adornment(line: &str, first_ch: char) -> Result<Line, char> {
    if SECTION_RECOMMENDED_ADORNMENT.chars().any(|ch| ch == first_ch) {
        Ok(Line::Adornment {
            adornment_char: first_ch,
            adornment_length: line.trim().len(),
            err: None,
        })
    } else {
        Err(first_ch)
    }
}

fn detect_acceptable_adornment(line: &str, first_ch: char) -> Result<Line, char> {
    if SECTION_ACCEPTABLE_ADORNMENT.chars().any(|ch| ch == first_ch) {
        Ok(Line::Adornment {
            adornment_char: first_ch,
            adornment_length: line.trim().len(),
            err: Some(Error::SectionAdornmentCharacterNotRecommended),
        })
    } else {
        Err(first_ch)
    }
}

fn detect_same_char_in_line(line: &str) -> Result<char, char> {
    // If we pass an empty line, it should panic here. This is exactly
    // what we expect: blank line detection should always happen before
    // detecting anything else, it's ensured by recognize() function.
    let mut chs = line.chars();
    let first_ch = chs.next().unwrap();
    match chs.all(|ch| ch == first_ch) {
        true => Ok(first_ch),
        false => Err(first_ch),
    }
}

fn detect_adornment(line: &str) -> Option<Line> {
    detect_same_char_in_line(line)
        .and_then(|first_ch| detect_recommended_adornment(line, first_ch))
        .or_else(|first_ch| detect_acceptable_adornment(line, first_ch))
        .ok()
}

fn detect_itemized_bullet(line: &str) -> bool {
    match line.len() > 2 {
        false => false,
        true => {
            let mut chs = line.chars();
            let first_ch = chs.next().unwrap();
            let second_ch = chs.next().unwrap();
            ITEMIZED_BULLET.chars().any(|ch| ch == first_ch) && second_ch.is_whitespace()
        }
    }
}

fn is_itemized(line: &str) -> bool {
    detect_itemized_bullet(line)
}

fn detect_text(line: &str) -> Option<Line> {
    let trimmed = line.trim_left();
    let indention_count = line.len() - trimmed.len();
    Some(Line::Text {
        indention_count: indention_count,
        itemized_like: is_itemized(trimmed),
    })
}

pub fn recognize(content: &Vec<&str>, line_no: usize) -> Line {
    detect_blank_line(content[line_no])
        .or_else(|| detect_adornment(content[line_no]))
        .or_else(|| detect_text(content[line_no]))
        .or(Some(Line::EndOfFile))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recognize_blankline() {
        let test_content: Vec<_> = "   \n".lines().collect();
        let expected_token = Line::BlankLine;
        assert_eq!(recognize(&test_content, 0), expected_token);
    }

    #[test]
    fn test_recognize_continuous_text_lines() {
        let test_content = r#"
========
Test
========
        "#;
        let lines: Vec<_> = test_content.lines().collect();
        let expected = [Line::BlankLine,
                        Line::Adornment {
                            adornment_char: '=',
                            adornment_length: 8,
                            err: None,
                        },
                        Line::Text {
                            indention_count: 0,
                            itemized_like: false,
                        },
                        Line::Adornment {
                            adornment_char: '=',
                            adornment_length: 8,
                            err: None,
                        }];

        for i in 0..4 {
            assert_eq!(recognize(&lines, i), expected[i]);
        }
    }

    #[test]
    fn test_itemized_line() {
        let lines: Vec<_> = "    - itemized".lines().collect();
        let intented_itemized = Line::Text {
            indention_count: 4,
            itemized_like: true,
        };

        assert_eq!(recognize(&lines, 0), intented_itemized);
    }
}
