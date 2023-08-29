pub mod uuid {
    use rand::Rng;
    use std::fmt::Write;

    pub fn v4() -> String {
        let mut rng = rand::thread_rng();
        let mut uuid = [0u8; 16];
        rng.fill(&mut uuid);

        // Set the version and variant according to the UUID v4 specifications
        uuid[6] = (uuid[6] & 0x0F) | 0x40;
        uuid[8] = (uuid[8] & 0x3F) | 0x80;

        let mut output = String::with_capacity(36);

        for (i, byte) in uuid.into_iter().enumerate() {
            if [4, 6, 8, 10].contains(&i) {
                output.push('-')
            }
            write!(output, "{:02x}", byte).unwrap();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::uuid;
    use regex::Regex;

    #[test]
    fn test_uuid_v4() {
        let uuid = uuid::v4();
        let uuid2 = uuid::v4();

        // Test that the UUIDs are different
        assert_ne!(uuid, uuid2);

        // Use a regular expression to validate the UUID format
        let re = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();

        // Test that the UUIDs match the expected format
        assert!(re.is_match(&uuid));
        assert!(re.is_match(&uuid2));
    }
}
