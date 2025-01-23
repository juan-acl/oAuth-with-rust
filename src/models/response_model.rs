use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn error(code: i32, message: String) -> Self {
        ApiResponse {
            code: code,
            message: message,
            data: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn success(code: i32, message: String, data: T) -> Self {
        ApiResponse {
            code: code,
            message: message,
            data: Some(data),
        }
    }
}
