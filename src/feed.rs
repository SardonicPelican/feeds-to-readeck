// Copyright 2016 Francis Gagné
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Indented;

pub enum Feed {
    Atom(Box<atom_syndication::Feed>),
    Rss(Box<rss::Channel>),
}

impl FromStr for Feed {
    type Err = FeedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<atom_syndication::Feed>() {
            Ok(feed) => Ok(Feed::Atom(Box::new(feed))),
            Err(atom_error) => match s.parse::<rss::Channel>() {
                Ok(channel) => Ok(Feed::Rss(Box::new(channel))),
                Err(rss_error) => Err(FeedError {
                    atom_error,
                    rss_error,
                }),
            },
        }
    }
}

#[derive(Debug)]
pub struct FeedError {
    pub atom_error: atom_syndication::Error,
    pub rss_error: rss::Error,
}

impl Display for FeedError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "could not parse input as either Atom or RSS:\n  parsing as Atom failed with:\n    {}\n  parsing as RSS failed with:\n    {}",
            Indented(Indented(&self.atom_error)), Indented(Indented(&self.rss_error)))
    }
}

impl Error for FeedError {
    fn description(&self) -> &str {
        "could not parse input as either Atom or RSS"
    }
}
