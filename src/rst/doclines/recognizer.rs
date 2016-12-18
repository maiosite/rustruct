// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define public interface of docline module.

use super::line::Line;
use super::adornment::detect_adornment;
use super::blankline::detect_blank_line;
use super::text::detect_text;

/// Detect type of given line from given reStructuredText document.
pub fn recognize(content: &Vec<&str>, line_no: usize) -> Line {
    let line_text = content[line_no];
    detect_blank_line(line_text)
        .or_else(|| detect_adornment(line_text))
        .or_else(|| detect_text(line_text))
        .or(Some(Line::EndOfFile))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::line::Line;

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
