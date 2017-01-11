// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define detection functions for textlines.

use super::itemized::is_itemized;
use super::line::Line;

/// Detect text line.
pub fn detect_text(line: &str) -> Option<Line> {
    let trimmed = line.trim_left();
    let indention_count = line.len() - trimmed.len();
    Some(Line::Text {
        indention_count: indention_count,
        itemized_like: is_itemized(trimmed),
    })
}
