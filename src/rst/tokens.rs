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

// This file defines a line-based tokens used by parser to break an
// ReStructuredText document to be a set of tokens.

use super::error::Error;

pub const SECTION_VALID_ADORNMENT: &'static str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const SECTION_RECOMMENDED_ADORNMENT: &'static str = "=-`:.'\"~^_*+#";

pub enum Token {
    BlankLine {
        with_spaces: bool,
    },

    AdornmentLine {
        adornment_char: char,
        length: i32,
    },

    TextLine {
        indention: i32,
        content: String,
    },

    SectionTitle {
        with_overline: bool,
        with_underline: bool,
        overline_length: i32,
        underline_length: i32,
        adornment: char,
    },
}

pub struct LineTokenizer {
}

// A set of functions to determine the type of lines
pub fn detect_blank_line(line: &str) -> Option<Token> {
    if line.trim().len() == 0 {
        if line.len() > 0 {
            Some(Token::BlankLine { with_spaces: true })
        } else {
            Some(Token::BlankLine { with_spaces: false })
        }
    } else {
        None
    }
}

impl LineTokenizer {
    pub fn new(full_content: &String) -> LineTokenizer {
        LineTokenizer {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[allow(unused_variables)]
    fn test_detect_blank_line() {
        assert!(match detect_blank_line("") {
            Some(Token::BlankLine { with_spaces: false }) => true,
            _ => false,
        });

        assert!(match detect_blank_line("    ") {
            Some(Token::BlankLine { with_spaces: true }) => true,
            _ => false,
        });

        assert!(match detect_blank_line("non-empty") {
            None => true,
            _ => false
        });
    }
}
