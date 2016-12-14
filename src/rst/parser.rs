// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

use super::error::Error;
use super::doclines::recognize;
use super::super::doctree::elements::Document;

/// Main interface to geneate doctree from ReStructuredText document.
#[derive(Debug, Copy, Clone)]
pub struct ReStructuredText {
}

impl ReStructuredText {
    /// Create a new parser.
    pub fn new() -> ReStructuredText {
        ReStructuredText {}
    }

    /// Read ReStructuredText documentation and return an doctree.
    pub fn read_string(content: &String) -> Result<Document, Error> {
        let lines: Vec<_> = content.lines().collect();
        // TODO parsing logic goes here
        for line_no in 0..lines.len() {
            recognize(&lines, line_no);
        }
        Ok(Document {})
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[allow(unused_variables)]
    fn create_parser() {
        let rst = ReStructuredText::new();
    }
}
