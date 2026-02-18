use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T>{
    pub success : bool,
    pub data : Option<T>,
    pub error : Option<String>
}

impl<T> APIResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}