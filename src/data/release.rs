use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    name: String,
    utc_time: DateTime<Utc>,
}

impl Release {
    fn new() -> Release {
        Release { name: String::new(), utc_time: Utc::now() }
    }
}