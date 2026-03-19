pub struct Config {
    pub env: String,
}

impl Config {
    fn new(env: String) -> Self {
        Self { env }
    }
}
