//! Get content-types for rust-http from file extensions
//!
//! Simple Example:
//! ```rust
//! req.headers.content_type = get_content_type("txt");
//! ```

extern crate http;

pub use get_content_type = self::mimes::get_generated_content_type;
mod mimes;

