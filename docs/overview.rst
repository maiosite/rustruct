=======================
Rustruct Overview
=======================

Hey man. Welcome to Rustruct world!

Wait! I don't know you... So what's Rustruct?

Rustruct is a library to parse documents written in ReStructuredText
format. It also provide capability to convert the output to Html5
documentation, which can be view from browsers. It tries to follow the
same ReStructuredText spec, defined by Docutils_.

Rustruct is written in Rust_ programming language, which is released as
both static library and executable files.

A code snippet below shows an example of how to use Rustruct to
generate Html5 documentation from ReStructuredText document.

.. code:: rust

  use rustruct::{ReStructuredText, DocumentTree, ParseErrorInfo};
  use rustruct::{ReStructuredText, DocumentTree, ParseErrorInfo};
  use rustruct::html::Html5Formatter;

  fn write_html(doctree: &DocumentTree, output: &str) {
      let formatter = Html5Formatter::new(&doctree);
      formatter.save_file(output);
  }

  fn print_error(info: &ParseErrorInfo) {
      println!("{}", info.to_pretty_string());
  }

  fn main() {
      let mut content = String::new();
      {
          let f = try!(File::open("input.rst"));
          try!(f.read_to_string(&mut content));
      }
      let rst = ReStructuredText::new();
      let result = rst.read_string(content);
      match result {
          Ok(doctree) => write_html(&doctree, "output.html"),
          Err(info) => print_error(&info);
      }
  }

Motivation
=============

OK I understand. But why do you want to reinvent the wheel, since you
have mentioned Docutils_?

We may think of Rustruct a just-for-fun project, like a lot of other
projects we can find. For myself, it's more like a self-learning
session, learning to write programs in Rust_. I expect it can be a
really good tool for all writers using ReStructuredText, but let's talk
about it when it's really usable first.

.. _Rust: http://www.rust-lang.org
.. _Docutils: http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html

