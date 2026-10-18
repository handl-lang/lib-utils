//! `utils` — stdlib utility modules (`bool`, `math`, `str`) and the
//! aggregate capability surface.
//!
//! The `.hnl` sources are the single source of truth for each module's
//! interface; the Rust handls backing `math.*` and `str.*` live in the
//! stdlib op table (`plugin-lib-stdlib::ops::{math,strs}`), which registers
//! them by op name.

/// The `bool` module (`bool.and`, `bool.or`, …) — pure handl.
pub const BOOL_HNL: &str = include_str!("bool.hnl");

/// The `math` module interface (`math.add`, …).
pub const MATH_HNL: &str = include_str!("math.hnl");

/// The `str` module interface.
pub const STR_HNL: &str = include_str!("str.hnl");

/// The `utils` aggregate package interface (capability declarations).
pub const UTILS_HNL: &str = include_str!("utils.hnl");
