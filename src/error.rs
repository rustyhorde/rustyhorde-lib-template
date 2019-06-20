// Copyright (c) {{ "now" | date: "%Y" }} {{project-name}} developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `{{project-name}}` Error Handling
//!
//! Example
//! ```
//! ```

use std::error::Error;
use std::fmt;

/// A result that includes a `Error`
crate type Result<T> = ::std::result::Result<T, Err>;

/// An error thrown by `{{project-name}}`
#[derive(Debug)]
crate struct Err {
    /// The kind of error
    inner: ErrKind,
}

impl Error for Err {
    fn description(&self) -> &str {
        "echoes error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())?;

        if let Some(source) = self.inner.source() {
            write!(f, ": {}", source)?;
        }
        write!(f, ": {}", self.inner)
    }
}

macro_rules! external_error {
    ($error:ty, $kind:expr) => {
        impl From<$error> for Err {
            fn from(inner: $error) -> Self {
                Self {
                    inner: $kind(inner),
                }
            }
        }
    };
}

impl From<ErrKind> for Err {
    fn from(inner: ErrKind) -> Self {
        Self { inner }
    }
}

impl From<&str> for Err {
    fn from(inner: &str) -> Self {
        Self {
            inner: ErrKind::Str(inner.to_string()),
        }
    }
}

external_error!(clap::Error, ErrKind::Clap);
external_error!(std::io::Error, ErrKind::Io);
external_error!(String, ErrKind::Str);
external_error!(std::env::VarError, ErrKind::Var);

#[derive(Debug)]
crate enum ErrKind {
    Clap(clap::Error),
    Io(std::io::Error),
    Str(String),
    Var(std::env::VarError),
}

impl Error for ErrKind {
    fn description(&self) -> &str {
        match self {
            ErrKind::Clap(inner) => inner.description(),
            ErrKind::Io(inner) => inner.description(),
            ErrKind::Str(inner) => &inner[..],
            ErrKind::Var(inner) => inner.description(),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrKind::Clap(inner) => inner.source(),
            ErrKind::Io(inner) => inner.source(),
            ErrKind::Var(inner) => inner.source(),
            _ => None,
        }
    }
}

impl fmt::Display for ErrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())?;
        match self {
            ErrKind::Clap(inner) => write!(f, ": {}", inner),
            ErrKind::Io(inner) => write!(f, ": {}", inner),
            ErrKind::Var(inner) => write!(f, ": {}", inner),
            _ => write!(f, ""),
        }
    }
}

crate fn display_error(error: &dyn Error) {
    eprintln!("{}", error.description());

    if let Some(source) = error.source() {
        eprintln!(": {}", source);
    }
}