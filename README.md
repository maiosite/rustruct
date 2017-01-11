[![Build Status (master)](https://travis-ci.org/maiosite/rustruct.svg?branch=master)](https://travis-ci.org/maiosite/rustruct) [![Build Status (dev branch)](https://travis-ci.org/maiosite/rustruct.svg?branch=fuzhouch)](https://travis-ci.org/maiosite/rustruct) [![License](https://img.shields.io/crates/l/protobuf.svg)](https://github.com/maiosite/rustruct/blob/master/LICENSE)

# What is Rustruct?

Rustruct is a library that implements a parser for
[ReStructuredText](http://docutils.sourceforge.net) documentation format.
It implements APIs to read an ReStructuredText snippet in string format,
and converts it to a tree form, which contains all elements (paragraphs,
reference links, etc.) of input snippet, which allows developers to
understand the structure of documentation.

Rustruct also comes with APIs to provide an HTML writer, which reads
document tree and converts it a HTML document. The HTML document is
saved to an .html file, which can be opened from browsers.

Rustruct is distributed under MIT 2.0 License. The full text of license
is part of the source code at
[here](https://github.com/maiosite/rustruct/blob/master/LICENSE).

Rustruct is written in [Rust](https://www.rust-lang.org).

# Install and use Rustruct

So far Rustruct is still under development. So for users who want to use
it to process ReStructuredText, it is not useful yet. If you just want
a tool to solve your problem, you should install Python and add docutils
module via pip.

For developers who want to read the source code, you may download and
build from source code with the following steps:

1. Make sure you installed latest Rust development distribution from
   [official Rust site](https://www.rust-lang.org).

2. Make sure you have installed Git. If you have no idea about it,
   check out Github's wonderful
   [Bootcamp document](https://help.github.com/articles/set-up-git/)

3. Open a local command prompt, clone my repository.

4. Checkout 'fuzhouch' branch.

4. Build library with Rust's standard command: cargo build.


# Documentation

Unfortunately the API documentation is also under development. The
documentation will be published with first usable version (0.1.0).
