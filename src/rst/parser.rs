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

use super::super::doctree::elements::Document;
use super::error::Error;

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
