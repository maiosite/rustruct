// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define functions to normalize input strings.
//! Normalization usually means the following:
//! 1. Escapes (backslashes) are checked and take effect.
//! 2. All tabs are converted to spaces.
//! 3. All repeated spaces are reduced to 1, except heading spaces.
//! 4. Heading spaces are counted.

use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

static TABSTOP: &'static str = "        "; // 1 tab = 8 spaces

fn replace_tabs_with_spaces<'a>(line: &'a str) -> Cow<'a, str> {
    if line.contains('\t') {
        Cow::Owned(line.replace("\t", TABSTOP))
    } else {
        Cow::Borrowed(line)
    }
}

fn reduce_dup_spaces<'a>(line: &'a str) -> Cow<'a, str> {
    if line.contains(' ') {
        let mut space_hit = false;
        let mut space_reduced = String::with_capacity(line.len());
        for each_ch in line.chars() {
            if each_ch == ' ' {
                if !space_hit {
                    space_reduced.push(' ');
                    space_hit = true;
                }
            } else {
                space_hit = false;
                space_reduced.push(each_ch)
            }
        }
        Cow::Owned(space_reduced)
    } else {
        Cow::Borrowed(line)
    }
}

pub fn normalize<'a>(line: &'a str) -> Cow<'a, str> {
    let local_line = line.clone();
    let replaced = replace_tabs_with_spaces(local_line);
    let result = reduce_dup_spaces(&replaced);
    Cow::Owned(result.into_owned())
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn test_tabs_to_8_spaces() {
        let line = "\t";
        let result = super::replace_tabs_with_spaces(line);
        assert_eq!(result.deref().len(), 8);
        for each_ch in result.deref().chars() {
            assert_eq!(each_ch, ' ');
        }
    }

    #[test]
    fn test_normalize_tabs() {
        let line = "\t\t";
        let result = super::normalize(line);
        assert_eq!(result, " ");
    }
}

// REF (and acknowledgement!)
// http://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html
