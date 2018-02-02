
pub mod optional_duration_format {
    use time::Duration;
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;

    // TODO: Rework this implementation
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
        where D: Deserializer<'de>
    {
        let v = Value::deserialize(deserializer)?;

        if v.is_null() {
            return Ok(None)
        }

        match v.as_f64() {
            Some(f) => {
                let i = (f*1000.0).round() as i64;
                Ok(Some(Duration::milliseconds(i)))
            },
            None => {
                Ok(None)
            }
        }
    }
}
