#[derive(Debug, Clone)]
pub struct LinuxRlimit {
    pub r#type: String,
    pub hard: u64,
    pub soft: u64,
}

impl LinuxRlimit {
    // Constructor for LinuxRlimit
    pub fn new(r#type: String, hard: u64, soft: u64) -> Self {
        Self { r#type, hard, soft }
    }

    // Method to display the rlimit in a readable format
    pub fn display(&self) {
        println!("Type: {}, Hard: {}, Soft: {}", self.r#type, self.hard, self.soft);
    }
}

// Example usage
fn main() {
    let rlimit = LinuxRlimit::new("RLIMIT_NOFILE".to_string(), 4096, 1024);
    rlimit.display();
}
