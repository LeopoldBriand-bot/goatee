use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

pub trait ITestRepository {
    fn test(&self) -> Response;
}

pub struct TestRepository {}

impl ITestRepository for TestRepository {
    fn test(&self) -> Response {
        Response {
            message: "Hello world.".to_string(),
            status: false,
        }
    }
}
