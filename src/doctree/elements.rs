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

pub struct SectionNode {
    title: String,
    level: i8
}

pub enum DocElement {
    Document,
    Section(SectionNode),
    Transition,

    Paragraph,
    BulletList,
    EnumeratedList,
    DefinitionList,
    FieldList,
    OptionList,
    LiteralBlock,
    BlockQuote,
    DoctestBlock,
    GridTable,
    SimpleTable,
    FootNotes,
    Citation,
    HyperlinkTarget,
    Directive,
    SubsitutionDefinition,
    Comment
}

pub trait Traversal {
    fn get_first_child(&self) -> Option<DocElement>;
    fn get_next_sibling(&self) -> Option<DocElement>;
}

impl Traversal for SectionNode {
    fn get_first_child(&self) -> Option<DocElement> {
        None
    }

    fn get_next_sibling(&self) -> Option<DocElement> {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn create_section() {
        let sec = SectionNode {
            title: "123".to_string(),
            level: 1
        };
    }
}
