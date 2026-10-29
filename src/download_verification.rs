use crate::version::Version;
use clap::ValueEnum;
use std::fmt::Display;
use url::Url;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum DownloadVerification {
    /// Check the download against `SHASUMS256.txt`, and check that file's signature
    /// by a Node.js releaser
    #[default]
    Signature,
    /// Check the download against `SHASUMS256.txt` only, without its signature
    Checksum,
    /// Don't check the download
    None,
}

impl DownloadVerification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Signature => "signature",
            Self::Checksum => "checksum",
            Self::None => "none",
        }
    }
}

impl Display for DownloadVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

const NODEJS_ORG: &str = "nodejs.org";
const UNOFFICIAL_BUILDS: &str = "unofficial-builds.nodejs.org";

/// What to check for a download, from the setting and the mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Check {
    Signature,
    Checksum { reason: ChecksumOnly },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumOnly {
    /// The setting asks for checksums only
    Requested,
    /// The setting asks for signatures, but the mirror publishes none
    HostHasNoSignatures,
}

/// What a missing `SHASUMS256.txt(.asc)` means for the mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenMissing {
    /// The Node.js project always publishes it: lowering the setting is no fix
    Incident { host: &'static str },
    /// The mirror may not publish it: lowering the setting to this is a fix
    Lower(DownloadVerification),
    /// The release came out before Node.js signed its checksums
    PredatesSignatures,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    pub check: Check,
    pub when_missing: WhenMissing,
    /// Over plain HTTP, an unsigned checksum list can be changed with the download.
    pub plain_http: bool,
}

/// Whether nodejs.org has no `SHASUMS256.txt.asc` for `version`. Signing started
/// with v0.10.0; v0.8.23 to v0.8.28 came out later and are signed, v0.10.7 isn't.
pub fn predates_signed_checksums(version: &Version) -> bool {
    let Version::Semver(v) = version else {
        return false;
    };
    let old_line = v.major == 0 && v.minor < 10 && !(v.minor == 8 && v.patch >= 23);
    old_line || (v.major, v.minor, v.patch) == (0, 10, 7)
}

/// The checks for a download of `version` from `mirror` with the `mode` setting.
pub fn policy(mode: DownloadVerification, mirror: &Url, version: &Version) -> Policy {
    let host = mirror
        .host_str()
        .map(|host| host.strip_suffix('.').unwrap_or(host));
    let official_host = match host {
        Some(NODEJS_ORG) => Some(NODEJS_ORG),
        Some(UNOFFICIAL_BUILDS) => Some(UNOFFICIAL_BUILDS),
        _ => None,
    };
    let check = match mode {
        DownloadVerification::Signature if official_host == Some(UNOFFICIAL_BUILDS) => {
            Check::Checksum {
                reason: ChecksumOnly::HostHasNoSignatures,
            }
        }
        DownloadVerification::Signature => Check::Signature,
        DownloadVerification::Checksum => Check::Checksum {
            reason: ChecksumOnly::Requested,
        },
        DownloadVerification::None => Check::None,
    };
    let when_missing = match (official_host, check) {
        (Some(NODEJS_ORG), Check::Signature) if predates_signed_checksums(version) => {
            WhenMissing::PredatesSignatures
        }
        (Some(host), _) => WhenMissing::Incident { host },
        (None, Check::Signature) => WhenMissing::Lower(DownloadVerification::Checksum),
        (None, Check::Checksum { .. } | Check::None) => {
            WhenMissing::Lower(DownloadVerification::None)
        }
    };
    Policy {
        check,
        when_missing,
        plain_http: mirror.scheme() == "http",
    }
}

impl Policy {
    /// A warning to print before the download, when it is checked less than fully.
    pub fn warning(&self) -> Option<String> {
        let warning = match self.check {
            Check::Signature => return None,
            Check::Checksum {
                reason: ChecksumOnly::Requested,
            } => "download verification is set to `checksum`: the download is checked against SHASUMS256.txt, but no signature is checked".to_string(),
            Check::Checksum {
                reason: ChecksumOnly::HostHasNoSignatures,
            } => format!(
                "{UNOFFICIAL_BUILDS} publishes no signatures: the download is checked against SHASUMS256.txt only"
            ),
            Check::None => {
                return Some(
                    "download verification is set to `none`: the download is not checked"
                        .to_string(),
                )
            }
        };
        if self.plain_http {
            return Some(format!(
                "{warning}. Over plain HTTP, this protects against corruption, not tampering"
            ));
        }
        Some(warning)
    }
}

impl Display for WhenMissing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incident { host } => write!(
                f,
                "This is unexpected for {host}: it can mean a publishing incident or an attack. Retry later, and don't lower the download verification setting."
            ),
            Self::Lower(mode) => write!(
                f,
                "If this is a private mirror you control, set FNM_DOWNLOAD_VERIFICATION={mode}."
            ),
            Self::PredatesSignatures => write!(
                f,
                "This release predates signed checksums; the most fnm can check is SHASUMS256.txt (--download-verification=checksum)."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn policy_for(mode: DownloadVerification, mirror: &str) -> Policy {
        let version = Version::parse("20.0.0").unwrap();
        policy(mode, &Url::parse(mirror).unwrap(), &version)
    }

    #[test]
    fn test_policy_for_every_mode_and_host() {
        use DownloadVerification as Mode;
        let requested = Check::Checksum {
            reason: ChecksumOnly::Requested,
        };
        let no_signatures = Check::Checksum {
            reason: ChecksumOnly::HostHasNoSignatures,
        };
        let nodejs = WhenMissing::Incident { host: NODEJS_ORG };
        let unofficial = WhenMissing::Incident {
            host: UNOFFICIAL_BUILDS,
        };
        let cases = [
            (
                "https://nodejs.org/dist",
                Mode::Signature,
                Check::Signature,
                nodejs,
            ),
            ("https://nodejs.org/dist", Mode::Checksum, requested, nodejs),
            ("https://nodejs.org/dist", Mode::None, Check::None, nodejs),
            (
                "https://unofficial-builds.nodejs.org/download/release",
                Mode::Signature,
                no_signatures,
                unofficial,
            ),
            (
                "https://unofficial-builds.nodejs.org/download/release",
                Mode::Checksum,
                requested,
                unofficial,
            ),
            (
                "https://unofficial-builds.nodejs.org/download/release",
                Mode::None,
                Check::None,
                unofficial,
            ),
            (
                "https://npmmirror.com/mirrors/node",
                Mode::Signature,
                Check::Signature,
                WhenMissing::Lower(Mode::Checksum),
            ),
            (
                "https://npmmirror.com/mirrors/node",
                Mode::Checksum,
                requested,
                WhenMissing::Lower(Mode::None),
            ),
            (
                "https://npmmirror.com/mirrors/node",
                Mode::None,
                Check::None,
                WhenMissing::Lower(Mode::None),
            ),
            // Only the exact host is official, whatever the path or scheme.
            (
                "https://nodejs.org./dist",
                Mode::Signature,
                Check::Signature,
                nodejs,
            ),
            (
                "http://NODEJS.org/some/path",
                Mode::Signature,
                Check::Signature,
                nodejs,
            ),
            (
                "https://nodejs.org.example.com/dist",
                Mode::Signature,
                Check::Signature,
                WhenMissing::Lower(Mode::Checksum),
            ),
            (
                "http://localhost:8080",
                Mode::Signature,
                Check::Signature,
                WhenMissing::Lower(Mode::Checksum),
            ),
        ];
        for (mirror, mode, check, when_missing) in cases {
            let policy = policy_for(mode, mirror);
            assert_eq!(
                (policy.check, policy.when_missing),
                (check, when_missing),
                "{mode} for {mirror}"
            );
        }
    }

    #[test]
    fn test_missing_file_hint_never_offers_a_bypass_for_official_hosts() {
        for mode in DownloadVerification::value_variants() {
            let hint = policy_for(*mode, "https://nodejs.org/dist")
                .when_missing
                .to_string();
            assert!(!hint.contains("FNM_DOWNLOAD_VERIFICATION"), "{hint}");
            assert!(hint.contains("unexpected for nodejs.org"), "{hint}");
        }
        let hint = policy_for(DownloadVerification::Signature, "https://example.com/node")
            .when_missing
            .to_string();
        assert_eq!(
            hint,
            "If this is a private mirror you control, set FNM_DOWNLOAD_VERIFICATION=checksum."
        );
    }

    #[test]
    fn test_only_full_verification_has_no_warning() {
        for mode in DownloadVerification::value_variants() {
            let policy = policy_for(*mode, "https://nodejs.org/dist");
            assert_eq!(
                policy.warning().is_none(),
                *mode == DownloadVerification::Signature
            );
        }
    }

    #[test]
    fn test_releases_before_signed_checksums() {
        // From a sweep of nodejs.org/dist: every release with SHASUMS256.txt
        // but no SHASUMS256.txt.asc.
        let unsigned = ["0.1.14", "0.6.21", "0.8.0", "0.8.22", "0.9.12", "0.10.7"];
        let signed = [
            "0.8.23", "0.8.28", "0.10.0", "0.10.6", "0.10.8", "4.0.0", "24.12.0",
        ];
        for version in unsigned {
            assert!(
                predates_signed_checksums(&Version::parse(version).unwrap()),
                "{version}"
            );
        }
        for version in signed {
            assert!(
                !predates_signed_checksums(&Version::parse(version).unwrap()),
                "{version}"
            );
        }

        let mirror = Url::parse("https://nodejs.org/dist").unwrap();
        let old = Version::parse("0.10.7").unwrap();
        let hint = policy(DownloadVerification::Signature, &mirror, &old).when_missing;
        assert_eq!(hint, WhenMissing::PredatesSignatures);
        assert!(hint
            .to_string()
            .contains("--download-verification=checksum"));
        // Other mirrors keep their hints, and checksum mode on nodejs.org stays an incident.
        let other = Url::parse("https://example.com/node").unwrap();
        assert_eq!(
            policy(DownloadVerification::Signature, &other, &old).when_missing,
            WhenMissing::Lower(DownloadVerification::Checksum)
        );
        assert_eq!(
            policy(DownloadVerification::Checksum, &mirror, &old).when_missing,
            WhenMissing::Incident { host: NODEJS_ORG }
        );
    }

    #[test]
    fn test_checksums_over_plain_http_warn_about_tampering() {
        let https = policy_for(DownloadVerification::Checksum, "https://example.com/node");
        let http = policy_for(
            DownloadVerification::Signature,
            "http://unofficial-builds.nodejs.org/download/release",
        );
        assert!(!https.warning().unwrap().contains("tampering"));
        assert!(http
            .warning()
            .unwrap()
            .ends_with("protects against corruption, not tampering"));
        let signed = policy_for(DownloadVerification::Signature, "http://nodejs.org/dist");
        assert_eq!(signed.warning(), None);
    }
}
