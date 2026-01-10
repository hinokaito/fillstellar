#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Triangle,
}

// シミュレート内オブジェクト
pub struct GameObject {
    pub shape: ShapeType,
}