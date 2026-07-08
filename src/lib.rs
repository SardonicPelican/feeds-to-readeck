// Copyright 2016 Francis Gagné
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod feed;
pub mod readeck;

use std::fmt::{self, Display};

/// Wraps a type implementing Display
/// and adds two spaces after each line feed in its display output.
pub struct Indented<D: Display>(pub D);

impl<D: Display> Display for Indented<D> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use std::fmt::Write;
        write!(IndentedWrite(fmt), "{}", self.0)
    }
}

/// Intercepts writes to a `std::fmt::Formatter`
/// and adds two spaces after each line feed written to it.
struct IndentedWrite<'a: 'f, 'f>(&'f mut fmt::Formatter<'a>);

// The documentation recommends implementing std::io::Write,
// but that trait operates on a stream of bytes,
// whereas std::fmt::Write operates on string slices.
// Additionally, we call Formatter::write_str(),
// which returns a Result<(), std::fmt::Error>,
// which matches the signature of std::fmt::Write::write_str().
impl<'a: 'f, 'f> fmt::Write for IndentedWrite<'a, 'f> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut lines = s.split('\n');
        if let Some(line) = lines.next() {
            self.0.write_str(line)?;
            for line in lines {
                self.0.write_str("\n  ")?;
                self.0.write_str(line)?;
            }
        }

        Ok(())
    }
}
