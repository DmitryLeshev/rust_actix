use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize)]
pub struct Status2 {
    pub status2: String,
}
