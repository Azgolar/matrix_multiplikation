#![feature(portable_simd)]

pub mod matrix;
pub mod test;

pub mod algorithmen {
    pub mod crossbeam;
    pub mod manuell_sicher;
    pub mod manuell_unsicher;
    pub mod rayon;
    pub mod simd;
    pub mod simd_tiling;
    pub mod single;
    pub mod tiling;
    pub mod unroll;
}
