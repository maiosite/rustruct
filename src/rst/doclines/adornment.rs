// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

use super::line::Line;
use super::super::error::Error;

static SECTION_RECOMMENDED_ADORNMENT: &'static str = "=-`:.'\"~^_*+#";
static SECTION_ACCEPTABLE_ADORNMENT: &'static str = "!$%&(),/;<>?@[\\]{|}";

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

/// Detect adornment line (used by sections and transitions)
pub fn detect_adornment(line: &str) -> Option<Line> {
    detect_same_char_in_line(line)
        .and_then(|first_ch| detect_recommended_adornment(line, first_ch))
        .or_else(|first_ch| detect_acceptable_adornment(line, first_ch))
        .ok()
}
