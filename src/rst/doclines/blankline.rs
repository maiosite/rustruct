// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define detection functions for blank lines.

use super::line::Line;

/// Detect blank line when parsing reStructuredText document.
pub fn detect_blank_line(line: &str) -> Option<Line> {
    if line.trim().len() == 0 {
        Some(Line::BlankLine)
    } else {
        None
    }
}
