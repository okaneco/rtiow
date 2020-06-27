//! Rust port of Peter Shirley's *Ray Tracing in One Weekend*.
#![warn(missing_docs, rust_2018_idioms, unsafe_code)]
#![allow(clippy::suspicious_arithmetic_impl, clippy::suspicious_op_assign_impl)]

pub mod camera;
pub mod conversion;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod render;
pub mod scene;
pub mod vec3;
