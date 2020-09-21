use serde::Serialize;

#[derive(Serialize)]
pub struct Upcheck {
    status: String,
}

pub fn upcheck() -> Upcheck {
    Upcheck {
        status: "OK".to_string(),
    }
}
