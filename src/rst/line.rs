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

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Line {
    BlankLine { line_no: usize },

    Adornment {
        line_no: usize,
        adornment_char: char,
        adornment_length: usize,
        err: Option<Error>,
    },

    Text {
        line_no: usize,
        indention_count: usize,
        itemized_like: bool,
    },

    EndOfFile,
}

fn detect_blank_line(line: &str, line_no: usize) -> Option<Line> {
    if line.trim().len() == 0 {
        Some(Line::BlankLine { line_no: line_no })
    } else {
        None
    }
}

fn detect_adornment(line: &str, line_no: usize) -> Option<Line> {
    let mut chs = line.chars();
    // If we pass an empty line, it should panic here. This is exactly
    // what we expect: blank line detection should always happen before
    // detecting anything else.
    let first = chs.next().unwrap();
    if chs.all(|ch| ch == first) {
        if SECTION_RECOMMENDED_ADORNMENT.chars().any(|ch| ch == first) {
            Some(Line::Adornment {
                line_no: line_no,
                adornment_char: first,
                adornment_length: line.trim().len(),
                err: None,
            })
        } else {
            if SECTION_ACCEPTABLE_ADORNMENT.chars().any(|ch| ch == first) {
                Some(Line::Adornment {
                    line_no: line_no,
                    adornment_char: first,
                    adornment_length: line.trim().len(),
                    err: Some(Error::SectionAdornmentCharacterNotRecommended),
                })
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn detect_text(line: &str, line_no: usize) -> Option<Line> {
    let trimmed = line.trim_right();
    let indention_count = line.len() - trimmed.len();
    Some(Line::Text {
        line_no: line_no,
        indention_count: indention_count,
        itemized_like: false, // TODO Will fix later
    })
}


pub fn recognize(content: &Vec<&str>, line_no: usize) -> Line {
    detect_blank_line(content[line_no], line_no)
        .or_else(|| detect_adornment(content[line_no], line_no))
        .or_else(|| detect_text(content[line_no], line_no))
        .or(Some(Line::EndOfFile))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recognize_blankline() {
        let test_content: Vec<_> = "   \n".lines().collect();
        let expected_token = Line::BlankLine { line_no: 0 };
        assert_eq!(recognize(&test_content, 0), expected_token);
    }

    struct LineParserTestData {
        index: usize,
        input: String,
        expected_output: Line,
    }

    #[test]
    fn test_recognize_continuous_text_lines() {
        let test_content = r#"
========
Test
========
        "#;
        let lines: Vec<_> = test_content.lines().collect();

        let expected_line_0 = Line::BlankLine { line_no: 0 };

        let expected_line_1 = Line::Adornment {
            line_no: 1,
            adornment_char: '=',
            adornment_length: 8,
            err: None,
        };

        let expected_line_2 = Line::Text {
            line_no: 2,
            indention_count: 0,
            itemized_like: false,
        };

        let expected_line_3 = Line::Adornment {
            line_no: 3,
            adornment_char: '=',
            adornment_length: 8,
            err: None,
        };

        assert_eq!(recognize(&lines, 0), expected_line_0);
        assert_eq!(recognize(&lines, 1), expected_line_1);
        assert_eq!(recognize(&lines, 2), expected_line_2);
        assert_eq!(recognize(&lines, 3), expected_line_3);
    }

}
