#![feature(portable_simd)]

pub mod matrix;
pub mod test;

pub mod algorithmen {
    pub mod single;
    pub mod manuell;
    pub mod unroll;
    pub mod tiling;
    pub mod simd;
    pub mod rayon;
    pub mod crossbeam;
}