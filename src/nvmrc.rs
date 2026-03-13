use crate::user_version::UserVersion;
use std::str::FromStr;

pub struct Nvmrc {
    content: String,
}

impl Nvmrc {
    pub fn parse(content: String) -> Option<Self> {
        Some(Self { content })
    }

    pub fn version(self) -> Option<UserVersion> {
        let cleaned = Self::strip_comments(&self.content);
        if cleaned.is_empty() {
            return None;
        }
        UserVersion::from_str(&cleaned).ok()
    }

    fn strip_comments(content: &str) -> String {
        content
            .lines()
            .map(|line| {
                // Remove everything after # (including #)
                line.split('#').next().unwrap_or("")
            })
            .map(|line| line.trim())
            .find(|line| !line.is_empty())
            .unwrap_or("")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_comments_with_leading_comment() {
        let content = "# This is a comment\n20.16.0";
        assert_eq!(Nvmrc::strip_comments(content), "20.16.0");
    }

    #[test]
    fn test_strip_comments_with_inline_comment() {
        let content = "20.16.0 # this is Node 20 LTS";
        assert_eq!(Nvmrc::strip_comments(content), "20.16.0");
    }

    #[test]
    fn test_strip_comments_with_multiple_comments() {
        let content = "# First comment\n# Second comment\n18.0.0\n# Trailing comment";
        assert_eq!(Nvmrc::strip_comments(content), "18.0.0");
    }

    #[test]
    fn test_strip_comments_with_lts_alias() {
        let content = "lts/iron\n\n# (maps to Node v20 LTS)";
        assert_eq!(Nvmrc::strip_comments(content), "lts/iron");
    }

    #[test]
    fn test_strip_comments_with_empty_lines() {
        let content = "\n\n# comment\n\nv21.x.x\n\n# another comment";
        assert_eq!(Nvmrc::strip_comments(content), "v21.x.x");
    }

    #[test]
    fn test_strip_comments_with_whitespace() {
        let content = "  # comment with spaces  \n  20.16.0  ";
        assert_eq!(Nvmrc::strip_comments(content), "20.16.0");
    }

    #[test]
    fn test_strip_comments_only_comments() {
        let content = "# comment 1\n# comment 2\n# comment 3";
        assert_eq!(Nvmrc::strip_comments(content), "");
    }

    #[test]
    fn test_strip_comments_empty_string() {
        let content = "";
        assert_eq!(Nvmrc::strip_comments(content), "");
    }

    #[test]
    fn test_strip_comments_whitespace_only() {
        let content = "   \n\n   \n";
        assert_eq!(Nvmrc::strip_comments(content), "");
    }

    #[test]
    fn test_strip_comments_no_comments() {
        let content = "20.16.0";
        assert_eq!(Nvmrc::strip_comments(content), "20.16.0");
    }

    #[test]
    fn test_strip_comments_multiple_versions_takes_first() {
        let content = "20.16.0\n18.0.0";
        assert_eq!(Nvmrc::strip_comments(content), "20.16.0");
    }

    #[test]
    fn test_strip_comments_with_v_prefix() {
        let content = "# Use Node 20\nv20.16.0 # Latest LTS";
        assert_eq!(Nvmrc::strip_comments(content), "v20.16.0");
    }

    #[test]
    fn test_version_parsing() {
        let nvmrc = Nvmrc::parse("# comment\n20.16.0".to_string()).unwrap();
        assert!(nvmrc.version().is_some());
    }

    #[test]
    fn test_version_parsing_with_inline_comment() {
        let nvmrc = Nvmrc::parse("20.16.0 # comment".to_string()).unwrap();
        assert!(nvmrc.version().is_some());
    }

    #[test]
    fn test_version_parsing_only_comments() {
        let nvmrc = Nvmrc::parse("# only comments".to_string()).unwrap();
        assert!(nvmrc.version().is_none());
    }
}
