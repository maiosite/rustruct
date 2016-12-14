// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define types of lines we may see when parsing reStructuredText.

use super::super::error::Error;

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
