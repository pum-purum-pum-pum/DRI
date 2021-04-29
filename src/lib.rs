#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

pub mod kms;

//use kms::{drm_screen_height, drm_screen_width, init, swap_buffers};

pub mod rand;
pub use rand::*;

pub mod egl;
pub mod gl3;

pub mod query_stab;
pub use query_stab::*;

pub use gl3 as gl;

