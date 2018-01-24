pub mod date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    // Stripped out nanoseconds from ISO 8601.
    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

pub mod optional_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    // Stripped out nanoseconds from ISO 8601.
    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if let Some(ref d) = *date {
            let s = format!("{}", &d.format(FORMAT));
            return serializer.serialize_str(&s)
        };
        serializer.serialize_none()
    }
}
