
#[derive(Debug)]
pub enum Replacement {
    EMPTY,
    STRING(StringReplacement),
    NUMBER(NumberReplacement),
    IMAGE(ImageReplacement),
    TABLE,
    LIST,
    CHECKBOX(CheckboxReplacement),
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

#[derive(Debug)]
pub struct CheckboxReplacement {
    pub value: bool,
}

#[derive(Debug)]
pub struct ImageReplacement {
    pub value: Vec<u8>,
    // pub width: f32,
    // pub height: f32,
}

