// Options module
//
// Public API surface for all graph configuration types.
// The module is split into three focused files:
//
//   charset.rs      — CharSet, DEFAULT_CHAR_SET, create_char_set
//   config.rs       — Config struct and its builder methods
//   annotations.rs  — ZeroLine, Threshold, StatAnnotations

mod charset;
mod config;
mod extensions;

// Re-export everything so the rest of the codebase can continue
// to use `crate::options::CharSet`, `crate::options::Config`, etc.
// without knowing about the internal file structure.
pub use charset::{CharSet, DEFAULT_CHAR_SET, create_char_set};
pub use config::Config;
pub use extensions::{ZeroLine, Threshold, StatAnnotations};