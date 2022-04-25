pub enum Trigger {
    StartWith(String), // Prefix
    EndWith(String),   // Prefix
    Contains(String),  // Prefix
    Equals(String),    // Prefix
}
impl Trigger {
    pub fn ed(&self, input: String) -> bool {
        match self {
            Trigger::StartWith(prefix) => {
                if input.len() >= prefix.len() {
                    input[0..prefix.len()] == *prefix
                } else {
                    false
                }
            }
            Trigger::EndWith(prefix) => {
                if input.len() >= prefix.len() {
                    input[input.len() - prefix.len()..input.len()] == *prefix
                } else {
                    false
                }
            }
            Trigger::Contains(prefix) => {
                if input.len() >= prefix.len() {
                    input.contains(prefix)
                } else {
                    false
                }
            }
            Trigger::Equals(prefix) => input == *prefix,
        }
    }
    // pub fn prefix(&self) -> &String {
    //     match self {
    //         Trigger::StartWith(prefix) => prefix,
    //         Trigger::EndWith(prefix) => prefix,
    //         Trigger::Contains(prefix) => prefix,
    //         Trigger::Equals(prefix) => prefix,
    //     }
    // }
}
