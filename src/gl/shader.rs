use super::*;
pub trait IShader {
    fn vertex(iface: i32, nthvert: i32) -> Vec3i;
    fn fragment(bar: Vec3f, color: TGAColor) -> bool;
}