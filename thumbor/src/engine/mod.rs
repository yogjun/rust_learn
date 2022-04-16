use crate::pb::Spec;
use image::ImageOutputFormat;
mod photon;
pub use photon::Photon;

pub trait Engine {
    // 对engine 按照specs 进行一些列有序处理
    fn apply(&mut self, specs: &[Spec]);
    // 从engine 中生成目标图片，使用self 而非self引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// spectransform:
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
