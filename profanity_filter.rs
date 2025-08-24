 struct Message {
        pub content: String,
        pub user: String,
    }
    
    impl Message {
        pub fn new(ms: String, u: String) -> Message {
            Message { content: ms, user: u }
        }
    
        pub fn send_ms(&self) -> Option<&str> {
            let c = self.content.trim();
            if c.is_empty() || c.contains("stupid") {
                None
            } else {
                Some(c)
            }
        }
    }
    
    // Optional: Keep the original function if you need Message-specific validation
    pub fn check_ms(ms: &Message) -> (bool, &str) {
        match ms.send_ms() {
            Some(v) => (true, v),
            None => (false, "ERROR: illegal"),
        }
    }
