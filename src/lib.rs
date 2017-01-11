// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

//! # RuStructuredText processing library
//! Convert ReStructuredText documentation to HTML documentation.

mod doctree;
mod rst;

pub mod html;

pub use rst::parser::ReStructuredText;
