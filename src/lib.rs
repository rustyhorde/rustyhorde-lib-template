// Copyright (c) {{ "now" | date: "%Y" }} {{project-name}} developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `{{project-name}}` 0.1.0
//!
//! Example
//! ```
//! ```
#![feature(crate_visibility_modifier)]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    clippy::all,
    clippy::pedantic,
    dead_code,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    question_mark_macro_sep,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(unsafe_code)]
#![allow(single_use_lifetimes)]
#![doc(html_root_url = "https://docs.rs/{{project-name}}/0.1.0")]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
