// 最小占位 schemas 模块，按需扩展
#[derive(Debug, Clone)]
pub struct Schema {
    pub name: &'static str,
}

impl Schema {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

pub fn default_schema() -> Schema {
    Schema::new("default")
}