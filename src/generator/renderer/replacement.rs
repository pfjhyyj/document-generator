
#[derive(Debug)]
pub enum Replacement {
    EMPTY,
    STRING(StringReplacement),
    NUMBER(NumberReplacement),
    IMAGE,
    TABLE,
    LIST,
    CHECKBOX,
    TEMPLATE,
}

#[derive(Debug)]
pub struct StringReplacement {
    pub value: String,
}

#[derive(Debug)]
pub struct NumberReplacement {
    pub value: f64,
    pub precision: u8,
}