use serde_json::Value;

pub enum Expression {
    Get(Label),
    Set(SetExpression),
    Delete(Label),
}

pub struct SetExpression {
    pub doc: (String, String),
    pub data: Value,
}
pub enum Label {
    Index(String),
    Doc((String,String))
}

