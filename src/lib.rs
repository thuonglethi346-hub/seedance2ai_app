/// Returns the official Seedance2AI website URL.
pub const fn homepage() -> &'static str {
    "https://www.seedance2ai.app"
}

/// Short product description used by downstream examples and tests.
pub const fn summary() -> &'static str {
    "Seedance2AI is an AI video generator for text-to-video and image-to-video workflows."
}

#[cfg(test)]
mod tests {
    use super::{homepage, summary};

    #[test]
    fn homepage_uses_https() {
        assert!(homepage().starts_with("https://"));
    }

    #[test]
    fn summary_mentions_video() {
        assert!(summary().contains("video"));
    }
}
