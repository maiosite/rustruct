// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

#[derive(PartialEq)]
#[derive(Debug,Clone,Copy)]
pub enum Error {
    SectionOverlineAndUnderlineUnmatched,
    SectionUnderlineMissing,
    SectionAdornmentLineTooShort,
    SectionAdornmentCharacterNotRecommended,
}
