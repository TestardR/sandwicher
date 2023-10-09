#[derive(Debug)]
pub enum CreateError {
    Unknown(String),
    InvalidData(String)
}

#[derive(Debug)]
pub enum GetError {
    Unknown(String),
    NotFound,
}
