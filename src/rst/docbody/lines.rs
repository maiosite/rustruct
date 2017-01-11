// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define types of lines we may see when parsing reStructuredText.

use super::super::error::Error;

enum EnumerationSequenceStyle {
    Arabic { value: i32 },
    UppercaseAlphabet { value: i32 },
    LowercaseAlphabet { value: i32 },
    UppercaseRoman { value: i32 },
    LowercaseRoman { value: i32 },
}

enum EnumerationFormattingStyle {
    SuffixedWithPeriodid,
    SuffixedWithRightParenthesis,
    SurroundedByParentheses,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Line {
    BlankLine,

    Adornment {
        style: char,
        length: usize,
        err: Option<Error>,
    },

    Text {
        estimated_indention_count: usize,
        indention_spaces: usize,
    },

    Bullet {
        style: String,
        indention_spaces: usize,
    },

    Enumeration {
        formatting: EnumerationFormattngStyle,
        sequence: EnumerationSequenceStyle,
        indention_spaces: usize,
    },

    EndOfFile,
}

pub fn normalize(content: &Vec<&str>, line_no: usize) -> String {
}

pub fn recognize(content: &Vec<&str>, line_no: usize) -> Line {
}
