// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Module to define methods used by

mod line;
mod text;
mod adornment;
mod blankline;
mod itemized;
mod recognizer;
mod preprocessing;

pub use self::line::Line;
pub use self::recognizer::recognize;
