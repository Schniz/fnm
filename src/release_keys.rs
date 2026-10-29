//! The Node.js release signing keys fnm trusts, and the check of a release's
//! `SHASUMS256.txt.asc` against them.

use log::debug;
use pgp::composed::{
    CleartextSignedMessage, Deserializable, SignedKeyDetails, SignedPublicKey, SignedPublicSubKey,
};
use pgp::packet::{KeyFlags, Signature, SignatureType, SubpacketData};
use pgp::types::{Fingerprint, KeyDetails, Tag};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;
use url::Url;

/// A snapshot of <https://github.com/nodejs/release-keys>, made by `.ci/update-release-keys.sh`.
const BUNDLED_KEYS: &str = include_str!("../release-keys/keys.asc");

/// The primary key that signed each release, and the active releasers' keys.
const SIGNERS: &str = include_str!("../release-keys/signers.txt");
const ACTIVE_KEYS: &str = include_str!("../release-keys/active.txt");

/// Node.js releases carry one signature; more than this is not worth hashing for.
const MAX_SIGNATURES: usize = 8;

/// Where keys are refreshed from: GitHub, never the Node.js mirror.
pub const RELEASE_KEYS_URL: &str =
    "https://codeload.github.com/nodejs/release-keys/tar.gz/refs/heads/main";

/// Limits for the release keys tarball, which is about 200 KiB, and for what
/// a refresh may add.
const MAX_TARBALL_SIZE: usize = 4 * 1024 * 1024;
const MAX_UNPACKED_SIZE: u64 = 16 * 1024 * 1024;
const MAX_KEY_FILE_SIZE: u64 = 256 * 1024;
const MAX_REFRESHED_SUBKEYS: usize = 64;

const CACHE_FILE: &str = "keys.asc";

/// Fetches a URL: [`fetch_from_github`], or a stand-in in tests.
pub type Fetch<'a> = &'a dyn Fn(&Url) -> Result<Vec<u8>, String>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("it is signed by key {issuer}, which is not a known Node.js release key")]
    UnknownIssuer { issuer: String },
    #[error("its signature is not valid: {reason}")]
    InvalidSignature { reason: String },
    #[error("SHASUMS256.txt.asc is signed by {}, which names itself \"{name}\" (unverified) and is newer than this fnm's key bundle. Upgrade fnm to get the key. If you can't wait: check the key's full fingerprint against https://github.com/nodejs/node#release-keys and https://github.com/nodejs/release-keys yourself, then add it with FNM_NODE_RELEASE_KEYS=<file>. Caution: whoever controls a key in that file can sign any Node.js download for you.", signed_by(.fingerprint, .signing_key))]
    NewReleaseKey {
        name: String,
        fingerprint: String,
        signing_key: String,
    },
    #[error("it is signed by key {issuer}, which is not a known Node.js release key, and the keys can't be refreshed from {url}: {reason}", url = RELEASE_KEYS_URL)]
    RefreshFailed { issuer: String, reason: String },
    #[error("the release keys in {} can't be used: {reason}", path.display())]
    UserKeys { path: PathBuf, reason: String },
    #[error("it is signed by {fingerprint}, but {version} was signed by {expected}")]
    WrongSigner {
        fingerprint: String,
        version: String,
        expected: String,
    },
    #[error("it is signed by retired key {fingerprint} ({name}); upgrade fnm")]
    RetiredKey { fingerprint: String, name: String },
}

fn signed_by(fingerprint: &str, signing_key: &str) -> String {
    if fingerprint == signing_key {
        format!("key {fingerprint}")
    } else {
        format!("subkey {signing_key} of key {fingerprint}")
    }
}

/// A release whose signature checked out.
#[derive(Debug)]
pub struct Verified {
    pub signer: String,
    pub fingerprint: String,
    /// The primary key or subkey that made the signature.
    pub signing_key: String,
    /// The `--node-release-keys` file the key is from, if not the bundle.
    pub user_keys: Option<PathBuf>,
    /// The signed text, the only place to take checksums from.
    pub signed_text: String,
}

enum Key<'a> {
    Primary(&'a SignedPublicKey),
    Subkey(&'a SignedPublicSubKey),
}

impl Key<'_> {
    fn fingerprint(&self) -> String {
        match self {
            Key::Primary(key) => format!("{:X}", key.fingerprint()),
            Key::Subkey(key) => format!("{:X}", key.fingerprint()),
        }
    }
}

struct TrustedKey {
    key: SignedPublicKey,
    name: String,
    /// New signing subkeys a refresh found for `key`, which is never changed.
    refreshed: Vec<SignedPublicSubKey>,
    user_keys: Option<PathBuf>,
    /// An active releaser's key, or a user key; retired keys only sign
    /// the releases they signed before.
    active: bool,
}

/// Which keys may sign a release.
struct Pin {
    version: String,
    /// The primary key that signed it, if the table of signers has it.
    signer: Option<String>,
}

struct Keyring {
    keys: Vec<TrustedKey>,
    pin: Option<Pin>,
}

impl Keyring {
    fn bundled() -> Self {
        let mut keyring = Self::from_keys(parse_keys(BUNDLED_KEYS));
        let active: Vec<_> = data_lines(ACTIVE_KEYS).collect();
        for trusted in &mut keyring.keys {
            trusted.active = active.contains(&format!("{:X}", trusted.key.fingerprint()).as_str());
        }
        keyring
    }

    /// Only the key that signed `version` before may sign it, or, for a
    /// release `signers` doesn't have, an active releaser's key.
    fn pin_to(&mut self, version: &str, signers: &str) {
        let signer = data_lines(signers).find_map(|line| {
            let (release, fingerprint) = line.split_once(' ')?;
            (release == version).then(|| fingerprint.to_string())
        });
        self.pin = Some(Pin {
            version: version.to_string(),
            signer,
        });
    }

    fn check_pin(&self, trusted: &TrustedKey) -> Result<(), Error> {
        let fingerprint = format!("{:X}", trusted.key.fingerprint());
        match &self.pin {
            None => Ok(()),
            Some(Pin {
                signer: Some(expected),
                ..
            }) if *expected == fingerprint => Ok(()),
            Some(Pin {
                version,
                signer: Some(expected),
            }) => Err(Error::WrongSigner {
                fingerprint,
                version: version.clone(),
                expected: expected.clone(),
            }),
            Some(Pin { signer: None, .. }) if trusted.active => Ok(()),
            Some(Pin { signer: None, .. }) => Err(Error::RetiredKey {
                fingerprint,
                name: trusted.name.clone(),
            }),
        }
    }

    fn from_keys(keys: Vec<SignedPublicKey>) -> Self {
        Self {
            keys: keys
                .into_iter()
                .map(|key| TrustedKey {
                    name: user_name(&key),
                    key,
                    refreshed: vec![],
                    user_keys: None,
                    active: true,
                })
                .collect(),
            pin: None,
        }
    }

    /// Adds the keys in the `--node-release-keys` file at `path`, trusted like
    /// bundled ones. A file without keys is an error, never ignored.
    fn add_user_keys(&mut self, path: &Path) -> Result<(), Error> {
        let error = |reason: String| Error::UserKeys {
            path: path.to_path_buf(),
            reason,
        };
        let armored = std::fs::read_to_string(path).map_err(|e| error(e.to_string()))?;
        // Unlike the bundle, a key in it that can't be read is an error.
        let blocks: Vec<_> = armor_blocks(&armored).collect();
        if armored.matches(PUBLIC_KEY_BEGIN).count() != blocks.len() {
            return Err(error("a key block in it doesn't end".to_string()));
        }
        let mut parsed = vec![];
        for block in blocks {
            parsed.extend(
                parse_block(block).map_err(|e| error(format!("a key in it can't be read: {e}")))?,
            );
        }
        let keys = Self::from_keys(parsed).keys;
        if keys.is_empty() {
            return Err(error("it has no OpenPGP public key".to_string()));
        }
        self.keys.extend(keys.into_iter().map(|key| TrustedKey {
            user_keys: Some(path.to_path_buf()),
            ..key
        }));
        Ok(())
    }

    fn contains(&self, fingerprint: &Fingerprint) -> bool {
        self.keys
            .iter()
            .any(|k| k.key.fingerprint() == *fingerprint)
    }

    fn refreshed(&self) -> impl Iterator<Item = &SignedPublicSubKey> {
        self.keys.iter().flat_map(|k| &k.refreshed)
    }

    /// Adds the subkeys of `keys` that are new signing subkeys of a trusted
    /// key: bound by the trusted primary key with the sign flag and the
    /// subkey's cross-signature, and not revoked. Only that binding is kept.
    fn add_refreshed(&mut self, keys: &[SignedPublicKey]) {
        for key in keys {
            for subkey in &key.public_subkeys {
                let fingerprint = subkey.fingerprint();
                let mut trusted_subkeys = self.keys.iter().flat_map(|k| &k.key.public_subkeys);
                if trusted_subkeys.any(|s| s.fingerprint() == fingerprint)
                    || self.refreshed().any(|s| s.fingerprint() == fingerprint)
                    || self.refreshed().count() >= MAX_REFRESHED_SUBKEYS
                {
                    continue;
                }
                let Some(trusted) = self
                    .keys
                    .iter_mut()
                    .find(|k| k.active && k.key.fingerprint() == key.fingerprint())
                else {
                    continue;
                };
                if let Some(binding) = signing_binding(&trusted.key, subkey, false) {
                    let signatures = vec![binding.clone()];
                    let key = subkey.key.clone();
                    trusted
                        .refreshed
                        .push(SignedPublicSubKey { key, signatures });
                }
            }
        }
    }

    /// Checks the clearsigned `asc` and returns its signed text. Text outside
    /// the signed block fails the check rather than being ignored.
    fn verify(&self, asc: &str) -> Result<Verified, Error> {
        let (message, _) =
            CleartextSignedMessage::from_string(asc).map_err(|e| Error::InvalidSignature {
                reason: e.to_string(),
            })?;
        self.verify_signatures(message.signatures(), message.signed_text())
    }

    fn verify_signatures(
        &self,
        signatures: &[Signature],
        signed_text: String,
    ) -> Result<Verified, Error> {
        // Signatures without an issuer are tried against every key, so the
        // keyring, not the downloaded file, bounds the work.
        let mut attempts_left: usize = self
            .keys
            .iter()
            .map(|k| 1 + k.key.public_subkeys.len() + k.refreshed.len())
            .sum();
        let mut unknown_issuer = None;
        let mut failure = None;

        for signature in signatures.iter().take(MAX_SIGNATURES) {
            if !matches!(
                signature.typ(),
                Some(SignatureType::Binary | SignatureType::Text)
            ) {
                failure.get_or_insert_with(|| invalid("it is not a document signature"));
                continue;
            }
            let (candidates, unusable) = self.candidates(signature);
            if candidates.is_empty() {
                if let Some(key) = unusable {
                    failure.get_or_insert_with(|| {
                        invalid(&format!(
                            "it is signed by key {key}, which is revoked or may not sign"
                        ))
                    });
                } else {
                    unknown_issuer.get_or_insert_with(|| issuer(signature));
                }
                continue;
            }
            for (trusted, key) in candidates {
                let Some(left) = attempts_left.checked_sub(1) else {
                    break;
                };
                attempts_left = left;
                let checked = match key {
                    Key::Primary(key) => signature.verify(key, signed_text.as_bytes()),
                    Key::Subkey(key) => signature.verify(key, signed_text.as_bytes()),
                };
                if checked.is_ok() {
                    if let Err(pinned) = self.check_pin(trusted) {
                        failure.get_or_insert(pinned);
                        break;
                    }
                    return Ok(Verified {
                        signer: trusted.name.clone(),
                        fingerprint: format!("{:X}", trusted.key.fingerprint()),
                        signing_key: key.fingerprint(),
                        user_keys: trusted.user_keys.clone(),
                        signed_text,
                    });
                }
            }
            if has_issuer(signature) {
                failure.get_or_insert_with(|| invalid("it doesn't match the signed text"));
            } else {
                unknown_issuer.get_or_insert_with(|| issuer(signature));
            }
        }

        match (failure, unknown_issuer) {
            (Some(failure), _) => Err(failure),
            (None, Some(issuer)) => Err(Error::UnknownIssuer { issuer }),
            (None, None) => Err(invalid("it has no signature")),
        }
    }

    /// The keys and subkeys that `signature` names as its issuer, or all of
    /// them when it names none, if they may make it. Also the first named
    /// key that may not, because it is revoked or not a signing key.
    fn candidates(&self, signature: &Signature) -> (Vec<(&TrustedKey, Key<'_>)>, Option<String>) {
        let fingerprints = signature.issuer_fingerprint();
        let key_ids = signature.issuer_key_id();
        let any_issuer = !has_issuer(signature);
        let matches = |key: &dyn KeyDetails| {
            any_issuer
                || fingerprints.iter().any(|f| **f == key.fingerprint())
                || key_ids.iter().any(|id| **id == key.legacy_key_id())
        };

        let mut candidates = vec![];
        let mut unusable = None;
        for trusted in &self.keys {
            let key = &trusted.key;
            if matches(key) {
                if primary_can_sign(key) && !self.revoked(&key.fingerprint()) {
                    candidates.push((trusted, Key::Primary(key)));
                } else if !any_issuer {
                    unusable.get_or_insert_with(|| format!("{:X}", key.fingerprint()));
                }
            }
            let subkeys = key.public_subkeys.iter().map(|subkey| (subkey, true));
            let refreshed = trusted.refreshed.iter().map(|subkey| (subkey, false));
            for (subkey, allow_flagless) in subkeys.chain(refreshed) {
                if matches(subkey) {
                    if signing_binding(key, subkey, allow_flagless).is_some()
                        && !self.revoked(&key.fingerprint())
                        && !self.revoked(&subkey.fingerprint())
                    {
                        candidates.push((trusted, Key::Subkey(subkey)));
                    } else if !any_issuer {
                        unusable.get_or_insert_with(|| format!("{:X}", subkey.fingerprint()));
                    }
                }
            }
        }
        (candidates, unusable)
    }

    /// Whether a trusted copy of the key or subkey with `fingerprint` carries
    /// its primary key's revocation. Copies are checked one by one otherwise,
    /// and one copy without the revocation must not keep the key usable.
    fn revoked(&self, fingerprint: &Fingerprint) -> bool {
        self.keys.iter().any(|trusted| {
            let key = &trusted.key;
            (key.fingerprint() == *fingerprint && primary_revoked(key))
                || key.public_subkeys.iter().any(|subkey| {
                    subkey.fingerprint() == *fingerprint && subkey_revoked(key, subkey)
                })
        })
    }

    #[cfg(test)]
    fn without(mut self, fingerprint: &str) -> Self {
        self.keys
            .retain(|k| format!("{:X}", k.key.fingerprint()) != fingerprint);
        self
    }
}

fn invalid(reason: &str) -> Error {
    Error::InvalidSignature {
        reason: reason.to_string(),
    }
}

/// The lines of a release-keys data file, without comments.
fn data_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
}

fn has_issuer(signature: &Signature) -> bool {
    !signature.issuer_fingerprint().is_empty() || !signature.issuer_key_id().is_empty()
}

fn issuer(signature: &Signature) -> String {
    if let Some(fingerprint) = signature.issuer_fingerprint().first() {
        format!("{fingerprint:X}")
    } else if let Some(key_id) = signature.issuer_key_id().first() {
        key_id.to_string().to_ascii_uppercase()
    } else {
        "(unnamed)".to_string()
    }
}

/// The key flags `signature` sets, if it sets any: older keys predate them.
fn key_flags(signature: &Signature) -> Option<KeyFlags> {
    signature
        .config()?
        .hashed_subpackets()
        .find_map(|p| match &p.data {
            SubpacketData::KeyFlags(flags) => Some(flags.clone()),
            _ => None,
        })
}

/// Any revocation by the primary key retires it for good.
fn primary_revoked(key: &SignedPublicKey) -> bool {
    key.details.revocation_signatures.iter().any(|revocation| {
        revocation.typ() == Some(SignatureType::KeyRevocation)
            && revocation.verify_key(&key.primary_key).is_ok()
    })
}

fn subkey_revoked(key: &SignedPublicKey, subkey: &SignedPublicSubKey) -> bool {
    subkey.signatures.iter().any(|signature| {
        signature.typ() == Some(SignatureType::SubkeyRevocation)
            && signature
                .verify_subkey_binding(&key.primary_key, &subkey.key)
                .is_ok()
    })
}

/// Whether the primary key may sign: it isn't revoked, and its newest
/// self-signature with key flags allows signing.
fn primary_can_sign(key: &SignedPublicKey) -> bool {
    let primary = &key.primary_key;
    let direct = key
        .details
        .direct_signatures
        .iter()
        .filter(|signature| signature.verify_key(primary).is_ok());
    let certifications = key.details.users.iter().flat_map(|user| {
        user.signatures.iter().filter(|signature| {
            signature.is_certification()
                && signature
                    .verify_certification(primary, Tag::UserId, &user.id)
                    .is_ok()
        })
    });
    !primary_revoked(key)
        && direct
            .chain(certifications)
            .filter_map(|signature| Some((signature.created(), key_flags(signature)?)))
            .max_by_key(|(created, _)| *created)
            .is_none_or(|(_, flags)| flags.sign())
}

/// The primary key's newest binding of `subkey`, if `subkey` may sign: neither
/// is revoked, and the binding allows signing, with the subkey's own
/// signature in it. A binding without key flags predates them and is allowed
/// when `allow_flagless`.
fn signing_binding<'a>(
    key: &SignedPublicKey,
    subkey: &'a SignedPublicSubKey,
    allow_flagless: bool,
) -> Option<&'a Signature> {
    let primary = &key.primary_key;
    let by_primary = |signature: &&Signature| {
        signature
            .verify_subkey_binding(primary, &subkey.key)
            .is_ok()
    };
    if subkey_revoked(key, subkey) || primary_revoked(key) {
        return None;
    }
    let binding = subkey
        .signatures
        .iter()
        .filter(|signature| signature.typ() == Some(SignatureType::SubkeyBinding))
        .filter(by_primary)
        .max_by_key(|signature| signature.created())?;
    let signs = match key_flags(binding) {
        None => allow_flagless,
        Some(flags) => {
            flags.sign()
                && binding.embedded_signature().is_some_and(|backsig| {
                    backsig
                        .verify_primary_key_binding(&subkey.key, primary)
                        .is_ok()
                })
        }
    };
    signs.then_some(binding)
}

/// `text` without control characters, which could rewrite the terminal.
fn printable(text: &str) -> String {
    text.chars().filter(|c| !c.is_control()).collect()
}

/// The key's first user ID, as in `Name <e-mail>`.
fn user_name(key: &SignedPublicKey) -> String {
    key.details
        .users
        .first()
        .map(|user| printable(String::from_utf8_lossy(user.id.id()).trim()))
        .unwrap_or_default()
}

/// Checks the clearsigned `asc` of `version` against the bundled keys, the
/// ones in `user_keys`, and the subkeys refreshed for them before, pinned to
/// the key that signed `version`. A signature by an unknown key refreshes the
/// keys from [`RELEASE_KEYS_URL`] with `fetch` once, then checks again.
pub fn verify(
    asc: &str,
    version: &str,
    cache_dir: &Path,
    user_keys: Option<&Path>,
    fetch: Fetch<'_>,
) -> Result<Verified, Error> {
    let mut keyring = Keyring::bundled();
    keyring.pin_to(version, SIGNERS);
    if let Some(path) = user_keys {
        if same_file(path, &cache_dir.join(CACHE_FILE)) {
            return Err(Error::UserKeys {
                path: path.to_path_buf(),
                reason: "it is fnm's cache of refreshed keys, which fnm rewrites; keep your keys in another file".to_string(),
            });
        }
        keyring.add_user_keys(path)?;
    }
    verify_with(keyring, asc, cache_dir, fetch)
}

/// Whether `a` and `b` are one file, or would be once `b` exists.
fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => matches!(
            (std::path::absolute(a), std::path::absolute(b)),
            (Ok(a), Ok(b)) if a == b
        ),
    }
}

fn verify_with(
    mut keyring: Keyring,
    asc: &str,
    cache_dir: &Path,
    fetch: Fetch<'_>,
) -> Result<Verified, Error> {
    let mut cached = String::new();
    if let Ok(file) = std::fs::File::open(cache_dir.join(CACHE_FILE)) {
        let limit = MAX_KEY_FILE_SIZE * MAX_REFRESHED_SUBKEYS as u64;
        let _ = file.take(limit).read_to_string(&mut cached);
    }
    keyring.add_refreshed(&parse_keys(&cached));
    let issuer = match keyring.verify(asc) {
        Err(Error::UnknownIssuer { issuer }) => issuer,
        result => return result,
    };

    debug!("Key {issuer} is not a known release key, refreshing the keys");
    let refreshed = refresh(fetch).map_err(|reason| Error::RefreshFailed {
        issuer: issuer.clone(),
        reason,
    })?;
    // The latest release keys decide: a subkey they revoked or dropped goes.
    for trusted in &mut keyring.keys {
        trusted.refreshed.clear();
    }
    keyring.add_refreshed(&refreshed.keys);
    if let Err(e) = write_cache(cache_dir, &refreshed.source, &keyring) {
        debug!("Can't cache the refreshed release keys: {e}");
    }

    match keyring.verify(asc) {
        Err(Error::UnknownIssuer { issuer }) => {
            // A releaser's key that isn't in the bundle is named, not trusted,
            // and only by subkeys it bound like a refreshed one.
            let new: Vec<_> = refreshed
                .keys
                .into_iter()
                .filter(|key| !keyring.contains(&key.fingerprint()))
                .collect();
            let primaries = new.iter().map(|key| SignedPublicKey {
                public_subkeys: vec![],
                ..key.clone()
            });
            let mut new_keys = Keyring::from_keys(primaries.collect());
            new_keys.add_refreshed(&new);
            match new_keys.verify(asc) {
                Ok(signed) => Err(Error::NewReleaseKey {
                    name: signed.signer,
                    fingerprint: signed.fingerprint,
                    signing_key: signed.signing_key,
                }),
                Err(_) => Err(Error::UnknownIssuer { issuer }),
            }
        }
        result => result,
    }
}

/// Fetches [`RELEASE_KEYS_URL`], at most once per process when it succeeds.
pub fn fetch_from_github(url: &Url) -> Result<Vec<u8>, String> {
    static FETCHED: OnceLock<Vec<u8>> = OnceLock::new();
    if let Some(body) = FETCHED.get() {
        return Ok(body.clone());
    }
    let response = crate::http::get(url.as_str()).map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status().as_u16()));
    }
    let mut body = vec![];
    response
        .take(MAX_TARBALL_SIZE as u64 + 1)
        .read_to_end(&mut body)
        .map_err(|e| e.to_string())?;
    Ok(FETCHED.get_or_init(|| body).clone())
}

struct Refreshed {
    /// The nodejs/release-keys commit, as recorded in the tarball.
    source: String,
    keys: Vec<SignedPublicKey>,
}

/// The keys in a nodejs/release-keys tarball. It comes from the network, so
/// only `keys/*.asc` files are read, each and all within size limits.
fn refresh(fetch: Fetch<'_>) -> Result<Refreshed, String> {
    let tarball = fetch(&Url::parse(RELEASE_KEYS_URL).expect("valid URL"))?;
    if tarball.len() > MAX_TARBALL_SIZE {
        return Err(format!(
            "the download is larger than {MAX_TARBALL_SIZE} bytes"
        ));
    }
    let unpacked = flate2::read::GzDecoder::new(tarball.as_slice()).take(MAX_UNPACKED_SIZE);
    let mut archive = tar::Archive::new(unpacked);
    let mut source = None;
    let mut keys = vec![];

    for entry in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_pax_global_extensions() {
            // `git archive` records the commit as a global `comment`.
            let extensions = entry.pax_extensions().map_err(|e| e.to_string())?;
            for extension in extensions.into_iter().flatten().flatten() {
                if extension.key() == Ok("comment") {
                    source = extension.value().ok().map(str::to_string);
                }
            }
            continue;
        }
        let path = entry.path().map_err(|e| e.to_string())?.into_owned();
        if !entry_type.is_file() || !is_key_file(&path) {
            continue;
        }
        if entry.size() > MAX_KEY_FILE_SIZE {
            debug!("Ignoring {}: it is too large", path.display());
            continue;
        }
        let mut armored = String::new();
        if let Err(e) = entry.read_to_string(&mut armored) {
            debug!("Ignoring {}: {e}", path.display());
            continue;
        }
        keys.extend(parse_keys(&armored));
    }

    let source = source
        .filter(|sha| sha.len() == 40 && sha.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| "unknown".to_string());
    Ok(Refreshed { source, keys })
}

/// `release-keys-<ref>/keys/<name>.asc`, and nothing else.
fn is_key_file(path: &Path) -> bool {
    let components: Vec<_> = path.components().collect();
    let [Component::Normal(root), Component::Normal(dir), Component::Normal(file)] =
        components.as_slice()
    else {
        return false;
    };
    root.to_string_lossy().starts_with("release-keys-")
        && *dir == "keys"
        && Path::new(file).extension().is_some_and(|ext| ext == "asc")
}

/// Writes the refreshed subkeys, each under its trusted primary key, and
/// replaces the cache in one rename, so a reader never sees half of it.
fn write_cache(cache_dir: &Path, source: &str, keyring: &Keyring) -> std::io::Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let mut file = tempfile::NamedTempFile::new_in(cache_dir)?;
    writeln!(
        file,
        "# Signing subkeys of bundled Node.js release keys, from https://github.com/nodejs/release-keys"
    )?;
    writeln!(file, "# Source commit: {source}")?;
    for trusted in keyring.keys.iter().filter(|k| !k.refreshed.is_empty()) {
        let key = SignedPublicKey {
            primary_key: trusted.key.primary_key.clone(),
            details: SignedKeyDetails::new(vec![], vec![], vec![], vec![]),
            public_subkeys: trusted.refreshed.clone(),
        };
        let armored = key
            .to_armored_string(None.into())
            .map_err(std::io::Error::other)?;
        writeln!(file, "\n{armored}")?;
    }
    file.persist(cache_dir.join(CACHE_FILE))
        .map_err(|e| e.error)?;
    Ok(())
}

/// Parses each armored key block on its own: one key that can't be parsed
/// is skipped instead of failing the keyring.
fn parse_keys(armored: &str) -> Vec<SignedPublicKey> {
    armor_blocks(armored)
        .flat_map(|block| {
            parse_block(block).unwrap_or_else(|e| {
                debug!("Skipping release keys that can't be parsed: {e}");
                vec![]
            })
        })
        .collect()
}

/// Every key in one armored block, or why it can't be read.
fn parse_block(block: &str) -> Result<Vec<SignedPublicKey>, String> {
    let (keys, _) = SignedPublicKey::from_string_many(block).map_err(|e| e.to_string())?;
    keys.map(|key| key.map_err(|e| e.to_string())).collect()
}

const PUBLIC_KEY_BEGIN: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----";
const PUBLIC_KEY_END: &str = "-----END PGP PUBLIC KEY BLOCK-----";

fn armor_blocks(armored: &str) -> impl Iterator<Item = &str> {
    let mut rest = armored;
    std::iter::from_fn(move || {
        let start = rest.find(PUBLIC_KEY_BEGIN)?;
        let end = rest[start..].find(PUBLIC_KEY_END)? + start + PUBLIC_KEY_END.len();
        let block = &rest[start..end];
        rest = &rest[end..];
        Some(block)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn fixture(version: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/release-signatures/{version}/SHASUMS256.txt.asc",
            env!("CARGO_MANIFEST_DIR")
        );
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_every_bundled_key_parses() {
        let count = BUNDLED_KEYS
            .lines()
            .find_map(|line| line.strip_prefix("# Keys: "))
            .and_then(|count| count.parse::<usize>().ok())
            .expect("keys.asc header names the key count");
        assert_eq!(Keyring::bundled().keys.len(), count);
    }

    #[test]
    fn test_a_broken_key_block_is_skipped() {
        let blocks: Vec<_> = armor_blocks(BUNDLED_KEYS).take(2).collect();
        let broken = format!(
            "{}\n-----BEGIN PGP PUBLIC KEY BLOCK-----\n\nnot a key\n-----END PGP PUBLIC KEY BLOCK-----\n{}",
            blocks[0], blocks[1]
        );
        assert_eq!(parse_keys(&broken).len(), 2);
    }

    #[test]
    fn test_releases_of_every_era_verify() {
        let keyring = Keyring::bundled();
        let cases = [
            // DSA-1024 key, SHA-1 signature, issuer key ID only.
            (
                "v0.10.30",
                "Timothy J Fontaine (OFTC) <tjfontaine@oftc.net>",
                "7937DFD2AB06298B2293C3187D33FF9D0246406D",
            ),
            // RSA subkey, issuer key ID only.
            (
                "v6.0.0",
                "James M Snell <jasnell@gmail.com>",
                "71DCFD284A79C3B38668286BC97EC7A07EDE3FC1",
            ),
            (
                "v20.0.0",
                "RafaelGSS <rafael.nunu@hotmail.com>",
                "890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4",
            ),
            // ed25519 subkey 86C8D74642E67846F8E120284DAA80D1E737BC9F.
            (
                "v24.12.0",
                "Michaël Zasso (Targos) <targos@protonmail.com>",
                "8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600",
            ),
            (
                "v25.2.0",
                "Antoine du Hamel <duhamelantoine1995@gmail.com>",
                "5BE8A3F6C8A5C01D106C0AD820B1A390B168D356",
            ),
        ];
        for (version, signer, fingerprint) in cases {
            let verified = keyring
                .verify(&fixture(version))
                .unwrap_or_else(|e| panic!("{version}: {e}"));
            assert_eq!(
                (verified.signer.as_str(), verified.fingerprint.as_str()),
                (signer, fingerprint),
                "{version}"
            );
            assert!(
                verified
                    .signed_text
                    .contains(&format!("node-{version}-linux-x64.tar.gz")),
                "{version}"
            );
        }
    }

    #[test]
    fn test_a_changed_checksum_fails() {
        let asc = fixture("v20.0.0");
        let line = asc
            .lines()
            .find(|line| line.ends_with("node-v20.0.0-linux-x64.tar.xz"))
            .unwrap();
        let tampered = asc.replace(line, &format!("{}{}", "0".repeat(64), &line[64..]));
        assert_ne!(tampered, asc);
        let error = Keyring::bundled().verify(&tampered).unwrap_err();
        assert!(matches!(error, Error::InvalidSignature { .. }), "{error}");
    }

    #[test]
    fn test_text_outside_the_signed_block_fails() {
        let asc = fixture("v20.0.0");
        let extra = format!("{}  node-v20.0.0-linux-x64.tar.xz\n", "0".repeat(64));
        for tampered in [format!("{extra}{asc}"), format!("{asc}{extra}")] {
            let error = Keyring::bundled().verify(&tampered).unwrap_err();
            assert!(matches!(error, Error::InvalidSignature { .. }), "{error}");
        }
    }

    #[test]
    fn test_an_unknown_issuer_is_named() {
        let keyring = Keyring::bundled().without("890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4");
        let error = keyring.verify(&fixture("v20.0.0")).unwrap_err();
        match error {
            Error::UnknownIssuer { issuer } => {
                // A full fingerprint, or the key ID that ends it.
                assert!(
                    "890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4".ends_with(&issuer),
                    "{issuer}"
                );
            }
            other => panic!("{other}"),
        }
    }

    #[test]
    fn test_a_signature_without_issuer_is_tried_against_every_key() {
        let (message, _) = CleartextSignedMessage::from_string(&fixture("v0.10.30")).unwrap();
        let mut signature = message.signatures()[0].clone();
        // The issuer key ID sits in the unhashed area, so removing it keeps the signature valid.
        while signature.unhashed_subpacket_remove(0).is_ok() {}
        assert!(signature.issuer_key_id().is_empty() && signature.issuer_fingerprint().is_empty());

        let verified = Keyring::bundled()
            .verify_signatures(&[signature], message.signed_text())
            .unwrap();
        assert_eq!(
            verified.fingerprint,
            "7937DFD2AB06298B2293C3187D33FF9D0246406D"
        );
    }

    #[test]
    fn test_signatures_past_the_limit_are_not_checked() {
        let (message, _) = CleartextSignedMessage::from_string(&fixture("v20.0.0")).unwrap();
        let (other, _) = CleartextSignedMessage::from_string(&fixture("v25.2.0")).unwrap();
        let mut signatures = vec![other.signatures()[0].clone(); MAX_SIGNATURES];
        signatures.push(message.signatures()[0].clone());
        // The only matching signature comes after MAX_SIGNATURES non-matching ones.
        let error = Keyring::bundled()
            .verify_signatures(&signatures, message.signed_text())
            .unwrap_err();
        assert!(matches!(error, Error::InvalidSignature { .. }), "{error}");
    }

    use pgp::bytes::Bytes;
    use pgp::composed::{SignedKeyDetails, SignedPublicSubKey};
    use pgp::crypto::hash::HashAlgorithm;
    use pgp::crypto::{ed25519, eddsa_legacy, public_key::PublicKeyAlgorithm};
    use pgp::packet::{
        PubKeyInner, PublicKey, PublicSubkey, RevocationCode, SecretKey, SecretSubkey,
        SignatureConfig, Subpacket, UserId,
    };
    use pgp::types::{
        KeyVersion, PacketHeaderVersion, Password, PlainSecretParams, PublicParams, SecretParams,
        SignedUser, SigningKey, Timestamp,
    };

    /// A release the table of signers doesn't have.
    const UNKNOWN_RELEASE: &str = "v99.0.0";

    /// Signed with by the tests below, after the signer's keys were made.
    const TEXT: &str = "3f4b5c0a2b1e7d9c8a6f5e4d3c2b1a0f9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b  node-v99.0.0-linux-x64.tar.xz\r\n";
    const AT: u32 = 1_800_000_000;

    fn key_pair(seed: u8) -> (PubKeyInner, SecretParams) {
        let secret =
            ed25519::SecretKey::try_from_bytes([seed; 32], ed25519::Mode::EdDSALegacy).unwrap();
        let public = PubKeyInner::new(
            KeyVersion::V4,
            PublicKeyAlgorithm::EdDSALegacy,
            Timestamp::from_secs(AT - 1_000_000),
            None,
            PublicParams::EdDSALegacy((&secret).into()),
        )
        .unwrap();
        let secret = PlainSecretParams::EdDSALegacy(eddsa_legacy::SecretKey::Ed25519(secret));
        (public, SecretParams::Plain(secret))
    }

    fn test_subkey(seed: u8) -> SecretSubkey {
        let (public, secret) = key_pair(seed);
        SecretSubkey::new(PublicSubkey::from_inner(public).unwrap(), secret).unwrap()
    }

    /// The secret keys of a test signer, so tests can make signatures of any kind.
    struct TestSigner {
        primary: SecretKey,
        subkey: SecretSubkey,
    }

    /// A key with a signing subkey, made from fixed seeds.
    fn test_signer() -> (TestSigner, SignedPublicKey) {
        let (public, secret) = key_pair(1);
        let primary_public = PublicKey::from_inner(public).unwrap();
        let primary = SecretKey::new(primary_public.clone(), secret).unwrap();
        let subkey = test_subkey(2);
        let subkey_public = subkey.public_key().clone();
        let pw = Password::empty();

        let user = UserId::from_str(
            PacketHeaderVersion::default(),
            "Test Signer <signer@example.com>",
        )
        .unwrap();
        let mut flags = KeyFlags::default();
        flags.set_certify(true);
        flags.set_sign(true);
        let mut certification = config(SignatureType::CertPositive, AT - 1000, &primary);
        certification
            .hashed_subpackets
            .push(Subpacket::regular(SubpacketData::KeyFlags(flags)).unwrap());
        let certification = certification
            .sign_certification(&primary, &primary_public, &pw, Tag::UserId, &user)
            .unwrap();
        let signer = TestSigner { primary, subkey };
        // Built by hand: `new` drops a subkey before its binding exists.
        let mut public = SignedPublicKey {
            primary_key: primary_public,
            details: SignedKeyDetails::new(
                vec![],
                vec![],
                vec![SignedUser::new(user, vec![certification])],
                vec![],
            ),
            public_subkeys: vec![SignedPublicSubKey {
                key: subkey_public,
                signatures: vec![],
            }],
        };
        rebind_subkey(&signer, &mut public, AT - 1000, Some(sign_only()), true);
        (signer, public)
    }

    fn sign_only() -> KeyFlags {
        let mut flags = KeyFlags::default();
        flags.set_sign(true);
        flags
    }

    fn keyring_of(key: SignedPublicKey) -> Keyring {
        keyring_of_all(vec![key])
    }

    fn keyring_of_all(keys: Vec<SignedPublicKey>) -> Keyring {
        Keyring::from_keys(keys)
    }

    fn config(typ: SignatureType, at: u32, issuer: &impl KeyDetails) -> SignatureConfig {
        let mut config = SignatureConfig::v4(typ, issuer.algorithm(), HashAlgorithm::Sha256);
        config.hashed_subpackets = vec![
            Subpacket::regular(SubpacketData::SignatureCreationTime(Timestamp::from_secs(
                at,
            )))
            .unwrap(),
            Subpacket::regular(SubpacketData::IssuerFingerprint(issuer.fingerprint())).unwrap(),
        ];
        config
    }

    fn sign(typ: SignatureType, at: u32, key: &impl SigningKey) -> Signature {
        config(typ, at, key)
            .sign(key, &Password::empty(), TEXT.as_bytes())
            .unwrap()
    }

    /// Like `sign`, for types rPGP verifies but won't make.
    fn sign_as(typ: SignatureType, key: &impl SigningKey) -> Signature {
        let config = config(typ, AT, key);
        let mut hasher = config.hash_alg.new_hasher().unwrap();
        config
            .hash_data_to_sign(&mut hasher, TEXT.as_bytes())
            .unwrap();
        let len = config.hash_signature_data(&mut hasher).unwrap();
        hasher.update(&config.trailer(len).unwrap());
        let hash = hasher.finalize();
        let signature = key
            .sign(&Password::empty(), config.hash_alg, &hash)
            .unwrap();
        Signature::from_config(config, [hash[0], hash[1]], signature).unwrap()
    }

    fn by_subkey(secret: &TestSigner, at: u32) -> Signature {
        sign(SignatureType::Text, at, &secret.subkey)
    }

    fn revoke_subkey(secret: &TestSigner, public: &mut SignedPublicKey, code: RevocationCode) {
        let mut config = config(SignatureType::SubkeyRevocation, AT, &secret.primary);
        config
            .hashed_subpackets
            .push(Subpacket::regular(SubpacketData::RevocationReason(code, Bytes::new())).unwrap());
        let revocation = config
            .sign_subkey_binding(
                &secret.primary,
                &public.primary_key,
                &Password::empty(),
                &public.public_subkeys[0].key,
            )
            .unwrap();
        public.public_subkeys[0].signatures.push(revocation);
    }

    fn rebind_subkey(
        signer: &TestSigner,
        public: &mut SignedPublicKey,
        at: u32,
        flags: Option<KeyFlags>,
        backsig: bool,
    ) {
        let subkey = &public.public_subkeys[0].key;
        let mut config = config(SignatureType::SubkeyBinding, at, &signer.primary);
        if let Some(flags) = flags {
            config
                .hashed_subpackets
                .push(Subpacket::regular(SubpacketData::KeyFlags(flags)).unwrap());
        }
        if backsig {
            let backsig = self::config(SignatureType::KeyBinding, at, &signer.subkey)
                .sign_primary_key_binding(
                    &signer.subkey,
                    subkey,
                    &Password::empty(),
                    &public.primary_key,
                )
                .unwrap();
            config.unhashed_subpackets.push(
                Subpacket::regular(SubpacketData::EmbeddedSignature(Box::new(backsig))).unwrap(),
            );
        }
        let binding = config
            .sign_subkey_binding(
                &signer.primary,
                &public.primary_key,
                &Password::empty(),
                subkey,
            )
            .unwrap();
        public.public_subkeys[0].signatures.push(binding);
    }

    fn rejection(keyring: &Keyring, signature: Signature) -> String {
        match keyring.verify_signatures(&[signature], TEXT.to_string()) {
            Err(Error::InvalidSignature { reason }) => reason,
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn test_only_document_signatures_are_accepted() {
        let (secret, public) = test_signer();
        let keyring = keyring_of(public.clone());
        let subkey = &secret.subkey;
        for typ in [SignatureType::Binary, SignatureType::Text] {
            let verified = keyring
                .verify_signatures(&[sign(typ, AT, subkey)], TEXT.to_string())
                .unwrap();
            assert_eq!(verified.signing_key, format!("{:X}", subkey.fingerprint()));
            assert_eq!(verified.fingerprint, format!("{:X}", public.fingerprint()));
        }
        // rPGP hashes only the first byte of the data for these, so one would
        // verify over any text that starts like this one.
        let standalone = sign_as(SignatureType::Standalone, subkey);
        assert!(standalone
            .verify(&public.public_subkeys[0], &b"3 anything"[..])
            .is_ok());
        for typ in [SignatureType::Standalone, SignatureType::Timestamp] {
            assert_eq!(
                rejection(&keyring, sign_as(typ, subkey)),
                "it is not a document signature"
            );
        }
    }

    #[test]
    fn test_a_revoked_subkey_cannot_sign_whatever_the_reason() {
        let (secret, public) = test_signer();
        keyring_of(public)
            .verify_signatures(&[by_subkey(&secret, AT - 100)], TEXT.to_string())
            .unwrap();
        // Even a superseded key: a signature's date can be forged with the key.
        for code in [
            RevocationCode::NoReason,
            RevocationCode::KeyCompromised,
            RevocationCode::KeySuperseded,
            RevocationCode::KeyRetired,
        ] {
            let (secret, mut public) = test_signer();
            revoke_subkey(&secret, &mut public, code);
            let reason = rejection(&keyring_of(public), by_subkey(&secret, AT - 100));
            assert!(
                reason.ends_with("which is revoked or may not sign"),
                "{reason}"
            );
        }
    }

    #[test]
    fn test_a_revocation_in_any_copy_of_a_key_applies_to_all() {
        let (secret, public) = test_signer();
        let mut revoked_subkey = public.clone();
        revoke_subkey(&secret, &mut revoked_subkey, RevocationCode::KeyCompromised);
        let mut revoked_primary = public.clone();
        let revocation = config(SignatureType::KeyRevocation, AT, &secret.primary)
            .sign_key(&secret.primary, &Password::empty(), &public.primary_key)
            .unwrap();
        revoked_primary
            .details
            .revocation_signatures
            .push(revocation);
        for revoked in [revoked_subkey, revoked_primary.clone()] {
            for keys in [
                vec![public.clone(), revoked.clone()],
                vec![revoked, public.clone()],
            ] {
                rejection(&keyring_of_all(keys), by_subkey(&secret, AT));
            }
        }
        let keyring = keyring_of_all(vec![public.clone(), revoked_primary]);
        rejection(&keyring, sign(SignatureType::Text, AT, &secret.primary));
    }

    #[test]
    fn test_a_revoked_primary_key_revokes_its_subkeys() {
        let (secret, mut public) = test_signer();
        let revocation = config(SignatureType::KeyRevocation, AT, &secret.primary)
            .sign_key(&secret.primary, &Password::empty(), &public.primary_key)
            .unwrap();
        public.details.revocation_signatures.push(revocation);
        let keyring = keyring_of(public);
        rejection(&keyring, by_subkey(&secret, AT));
        rejection(&keyring, sign(SignatureType::Text, AT, &secret.primary));
    }

    #[test]
    fn test_the_newest_binding_decides_whether_a_subkey_signs() {
        let (secret, mut public) = test_signer();
        let mut encrypt_only = KeyFlags::default();
        encrypt_only.set_encrypt_comms(true);
        rebind_subkey(&secret, &mut public, AT, Some(encrypt_only), true);
        rejection(&keyring_of(public), by_subkey(&secret, AT));

        // A signing subkey needs the subkey's own signature in its binding.
        let (secret, mut public) = test_signer();
        rebind_subkey(&secret, &mut public, AT, Some(sign_only()), false);
        rejection(&keyring_of(public), by_subkey(&secret, AT));
    }

    #[test]
    fn test_a_bundled_subkey_revoked_and_bound_again_stays_revoked() {
        let keyring = Keyring::bundled();
        let key = &keyring
            .keys
            .iter()
            .find(|k| {
                format!("{:X}", k.key.fingerprint()) == "C4F0DFFF4E8C1A8236409D08E73BC641CC11F4C8"
            })
            .unwrap()
            .key;
        let can_sign = |fingerprint: &str| {
            let subkey = key
                .public_subkeys
                .iter()
                .find(|s| format!("{:X}", s.fingerprint()) == fingerprint)
                .unwrap();
            signing_binding(key, subkey, true).is_some()
        };
        // Revoked in 2018-11-02 and bound again on 2018-11-20: gpg treats it as revoked.
        assert!(!can_sign("2B2B9F0D620463DB3DE74C50DEA16371974031A5"));
        // The subkey that signed every release by this key.
        assert!(can_sign("0EFFE1BCEFD9C84E3D098152933B01F40B5CA946"));
    }

    #[test]
    fn test_names_are_printed_without_control_characters() {
        assert_eq!(printable("Evil\x1b[2J\r\nName\x07"), "Evil[2JName");
    }
    // Made with gpg: ed25519 keys A and B, each given a signing subkey after
    // `bundle.asc` was exported. The `signed-by-*` files are clearsigned by those subkeys.

    /// Test Releaser A, in the test bundle without a subkey.
    const KNOWN_KEY: &str = "0B129F5056530532A02FDC3BA626C7F3E0572A62";
    /// Test Releaser B, not in the test bundle, and its signing subkey.
    const NEW_KEY: &str = "9B27DA36EE70011AB545B225110C812E19844D84";
    const NEW_SUBKEY: &str = "65738DBBFF995F482F731DF28A54FB5BD7767DCE";

    fn refresh_fixture(name: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/release-keys-refresh/{name}",
            env!("CARGO_MANIFEST_DIR")
        );
        std::fs::read_to_string(path).unwrap()
    }

    fn test_bundle() -> Keyring {
        Keyring::from_keys(parse_keys(&refresh_fixture("bundle.asc")))
    }

    /// A gzipped tarball like the one GitHub serves for nodejs/release-keys.
    fn tarball(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let gz = flate2::write::GzEncoder::new(vec![], flate2::Compression::fast());
        let mut builder = tar::Builder::new(gz);
        let comment = b"52 comment=481637f813e912c4aa3622d7964ab426c97b8e8d\n";
        let mut header = tar::Header::new_ustar();
        header.set_entry_type(tar::EntryType::XGlobalHeader);
        header.set_size(comment.len() as u64);
        builder
            .append_data(&mut header, "pax_global_header", &comment[..])
            .unwrap();
        for (path, data) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            builder.append_data(&mut header, path, *data).unwrap();
        }
        builder.into_inner().unwrap().finish().unwrap()
    }

    fn verify_refreshing(
        bundle: Keyring,
        asc: &str,
        cache_dir: &Path,
        tarball: &[u8],
    ) -> (Result<Verified, Error>, usize) {
        let calls = std::cell::Cell::new(0);
        let fetch = |url: &Url| {
            assert_eq!(url.as_str(), RELEASE_KEYS_URL);
            calls.set(calls.get() + 1);
            Ok(tarball.to_vec())
        };
        let result = verify_with(bundle, asc, cache_dir, &fetch);
        (result, calls.get())
    }

    fn no_fetch(_: &Url) -> Result<Vec<u8>, String> {
        panic!("the keys are not refreshed when the signer is known")
    }

    #[test]
    fn test_refresh_adds_a_new_subkey_of_a_bundled_key() {
        let asc = refresh_fixture("signed-by-A-subkey.asc");
        let cache = tempfile::tempdir().unwrap();
        assert!(matches!(
            test_bundle().verify(&asc),
            Err(Error::UnknownIssuer { .. })
        ));

        let with_subkey = refresh_fixture("A-with-subkey.asc");
        let tarball = tarball(&[(
            &format!("release-keys-main/keys/{KNOWN_KEY}.asc"),
            with_subkey.as_bytes(),
        )]);
        let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
        let verified = result.unwrap();
        assert_eq!(calls, 1);
        assert_eq!(verified.signer, "Test Releaser A <a@example.com>");
        assert_eq!(verified.fingerprint, KNOWN_KEY);

        let cached = std::fs::read_to_string(cache.path().join(CACHE_FILE)).unwrap();
        assert!(cached.contains("# Source commit: 481637f813e912c4aa3622d7964ab426c97b8e8d"));
        // The next check uses the cache, without fetching.
        verify_with(test_bundle(), &asc, cache.path(), &no_fetch).unwrap();
    }

    #[test]
    fn test_refresh_names_but_does_not_trust_a_new_release_key() {
        let asc = refresh_fixture("signed-by-B-subkey.asc");
        let cache = tempfile::tempdir().unwrap();
        let cached_before = refresh_fixture("A-with-subkey.asc");
        std::fs::write(cache.path().join(CACHE_FILE), &cached_before).unwrap();
        let new_key = refresh_fixture("B.asc");
        let tarball = tarball(&[(
            &format!("release-keys-main/keys/{NEW_KEY}.asc"),
            new_key.as_bytes(),
        )]);
        let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
        assert_eq!(calls, 1);
        let error = result.unwrap_err();
        let Error::NewReleaseKey {
            name,
            fingerprint,
            signing_key,
        } = &error
        else {
            panic!("{error}");
        };
        assert_eq!(
            (name.as_str(), fingerprint.as_str(), signing_key.as_str()),
            ("Test Releaser B <b@example.com>", NEW_KEY, NEW_SUBKEY)
        );
        assert_eq!(
            error.to_string(),
            format!(
                "SHASUMS256.txt.asc is signed by subkey {NEW_SUBKEY} of key {NEW_KEY}, which names itself \"Test Releaser B <b@example.com>\" (unverified) and is newer than this fnm's key bundle. Upgrade fnm to get the key. If you can't wait: check the key's full fingerprint against https://github.com/nodejs/node#release-keys and https://github.com/nodejs/release-keys yourself, then add it with FNM_NODE_RELEASE_KEYS=<file>. Caution: whoever controls a key in that file can sign any Node.js download for you."
            )
        );

        // The cache follows the latest keys, which don't have A's subkey, and not B's.
        let cached = std::fs::read_to_string(cache.path().join(CACHE_FILE)).unwrap();
        assert!(parse_keys(&cached).is_empty(), "{cached}");
    }

    #[test]
    fn test_refresh_drops_a_subkey_the_primary_key_did_not_bind() {
        let asc = refresh_fixture("signed-by-B-subkey.asc");
        let (mut forged, _) = SignedPublicKey::from_string(&refresh_fixture("bundle.asc")).unwrap();
        let (new_key, _) = SignedPublicKey::from_string(&refresh_fixture("B.asc")).unwrap();
        forged
            .public_subkeys
            .push(new_key.public_subkeys[0].clone());
        // Without the binding check, B's subkey would verify as A's.
        let (message, _) = CleartextSignedMessage::from_string(&asc).unwrap();
        assert!(message
            .verify(forged.public_subkeys.last().unwrap())
            .is_ok());

        let armored = forged.to_armored_string(None.into()).unwrap();
        let tarball = tarball(&[(
            &format!("release-keys-main/keys/{KNOWN_KEY}.asc"),
            armored.as_bytes(),
        )]);
        let cache = tempfile::tempdir().unwrap();
        let (result, _) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
        let error = result.unwrap_err();
        assert!(matches!(error, Error::UnknownIssuer { .. }), "{error}");
    }

    #[test]
    fn test_refresh_reads_only_key_files_within_limits() {
        let asc = refresh_fixture("signed-by-A-subkey.asc");
        let with_subkey = refresh_fixture("A-with-subkey.asc");
        let oversized = format!(
            "{with_subkey}{}",
            " ".repeat(usize::try_from(MAX_KEY_FILE_SIZE).unwrap())
        );
        let tarball = tarball(&[
            ("release-keys-main/README.md", with_subkey.as_bytes()),
            ("release-keys-main/gpg/key.asc", with_subkey.as_bytes()),
            ("release-keys-main/keys/old/key.asc", with_subkey.as_bytes()),
            ("release-keys-main/keys/key.txt", with_subkey.as_bytes()),
            ("other-repo/keys/key.asc", with_subkey.as_bytes()),
            ("release-keys-main/keys/oversized.asc", oversized.as_bytes()),
        ]);
        let cache = tempfile::tempdir().unwrap();
        let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
        assert_eq!(calls, 1);
        let error = result.unwrap_err();
        assert!(matches!(error, Error::UnknownIssuer { .. }), "{error}");
    }

    #[test]
    fn test_refresh_failures_are_reported() {
        let asc = refresh_fixture("signed-by-A-subkey.asc");
        let cache = tempfile::tempdir().unwrap();
        let unpacks_too_large = tarball(&[(
            "release-keys-main/README.md",
            &vec![0; usize::try_from(MAX_UNPACKED_SIZE).unwrap() + 1],
        )]);
        // Random bytes don't compress, so this is a valid tarball over the limit.
        let random: Vec<u8> = std::iter::repeat_with(|| fastrand::u8(..))
            .take(MAX_TARBALL_SIZE)
            .collect();
        let too_large = tarball(&[("release-keys-main/README.md", &random)]);
        assert!(too_large.len() > MAX_TARBALL_SIZE);
        for tarball in [too_large, b"not a tarball".to_vec(), unpacks_too_large] {
            let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
            assert_eq!(calls, 1);
            let error = result.unwrap_err();
            assert!(matches!(error, Error::RefreshFailed { .. }), "{error}");
        }

        let offline = |_: &Url| Err("offline".to_string());
        let error = verify_with(test_bundle(), &asc, cache.path(), &offline).unwrap_err();
        assert!(error.to_string().ends_with(": offline"), "{error}");
    }

    #[test]
    fn test_a_cached_key_that_is_not_bundled_is_ignored() {
        let asc = refresh_fixture("signed-by-B-subkey.asc");
        let cache = tempfile::tempdir().unwrap();
        std::fs::write(cache.path().join(CACHE_FILE), refresh_fixture("B.asc")).unwrap();
        let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball(&[]));
        assert_eq!(calls, 1);
        let error = result.unwrap_err();
        assert!(matches!(error, Error::UnknownIssuer { .. }), "{error}");
    }

    #[test]
    fn test_refresh_cannot_name_a_new_key_by_a_subkey_it_did_not_bind() {
        // A's subkey signed this; attached to B, it isn't B's.
        let asc = refresh_fixture("signed-by-A-subkey.asc");
        let (mut forged, _) = SignedPublicKey::from_string(&refresh_fixture("B.asc")).unwrap();
        let (with_subkey, _) =
            SignedPublicKey::from_string(&refresh_fixture("A-with-subkey.asc")).unwrap();
        forged
            .public_subkeys
            .push(with_subkey.public_subkeys[0].clone());
        let armored = forged.to_armored_string(None.into()).unwrap();
        let tarball = tarball(&[(
            &format!("release-keys-main/keys/{NEW_KEY}.asc"),
            armored.as_bytes(),
        )]);
        let cache = tempfile::tempdir().unwrap();
        let (result, _) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball);
        let error = result.unwrap_err();
        assert!(matches!(error, Error::UnknownIssuer { .. }), "{error}");
    }

    fn without_subkeys(key: &SignedPublicKey) -> SignedPublicKey {
        SignedPublicKey {
            public_subkeys: vec![],
            ..key.clone()
        }
    }

    fn armored(key: &SignedPublicKey) -> String {
        key.to_armored_string(None.into()).unwrap()
    }

    fn clearsigned_by_subkey(signer: &TestSigner) -> String {
        CleartextSignedMessage::new(
            TEXT,
            config(SignatureType::Text, AT, &signer.subkey),
            &signer.subkey,
            &Password::empty(),
        )
        .unwrap()
        .to_armored_string(None.into())
        .unwrap()
    }

    /// Refreshes the keys with `refreshed`, through a release by an unknown key.
    fn refresh_with(bundle: Keyring, cache_dir: &Path, refreshed: &SignedPublicKey) {
        let tarball = tarball(&[(
            "release-keys-main/keys/key.asc",
            armored(refreshed).as_bytes(),
        )]);
        let unknown = refresh_fixture("signed-by-B-subkey.asc");
        let (result, calls) = verify_refreshing(bundle, &unknown, cache_dir, &tarball);
        assert_eq!(calls, 1);
        assert!(matches!(result, Err(Error::UnknownIssuer { .. })));
    }

    #[test]
    fn test_a_refresh_never_changes_a_bundled_subkey() {
        let (signer, public) = test_signer();
        let asc = clearsigned_by_subkey(&signer);
        // The same binding without the subkey's cross-signature, which isn't signed.
        let mut stripped = public.clone();
        while stripped.public_subkeys[0].signatures[0]
            .unhashed_subpacket_remove(0)
            .is_ok()
        {}
        let mut revoked = public.clone();
        revoke_subkey(&signer, &mut revoked, RevocationCode::KeyCompromised);
        for refreshed in [stripped, revoked] {
            let cache = tempfile::tempdir().unwrap();
            refresh_with(keyring_of(public.clone()), cache.path(), &refreshed);
            let cached = std::fs::read_to_string(cache.path().join(CACHE_FILE)).unwrap();
            assert!(parse_keys(&cached).is_empty(), "{cached}");
            verify_with(keyring_of(public.clone()), &asc, cache.path(), &no_fetch).unwrap();
        }
    }

    #[test]
    fn test_a_refresh_adds_only_new_subkeys_that_may_sign() {
        let (signer, public) = test_signer();
        let asc = clearsigned_by_subkey(&signer);
        let bundle = || keyring_of(without_subkeys(&public));

        let mut stripped = public.clone();
        while stripped.public_subkeys[0].signatures[0]
            .unhashed_subpacket_remove(0)
            .is_ok()
        {}
        let mut revoked = public.clone();
        revoke_subkey(&signer, &mut revoked, RevocationCode::KeySuperseded);
        let mut flagless = without_subkeys(&public);
        flagless.public_subkeys = vec![SignedPublicSubKey {
            key: public.public_subkeys[0].key.clone(),
            signatures: vec![],
        }];
        rebind_subkey(&signer, &mut flagless, AT, None, true);
        for refreshed in [stripped, revoked, flagless] {
            let cache = tempfile::tempdir().unwrap();
            refresh_with(bundle(), cache.path(), &refreshed);
            let (result, _) = verify_refreshing(bundle(), &asc, cache.path(), &tarball(&[]));
            assert!(matches!(result, Err(Error::UnknownIssuer { .. })));
        }

        let cache = tempfile::tempdir().unwrap();
        refresh_with(bundle(), cache.path(), &public);
        verify_with(bundle(), &asc, cache.path(), &no_fetch).unwrap();
    }

    #[test]
    fn test_a_later_refresh_drops_a_revoked_subkey() {
        let (signer, public) = test_signer();
        let asc = clearsigned_by_subkey(&signer);
        let bundle = || keyring_of(without_subkeys(&public));
        let cache = tempfile::tempdir().unwrap();
        refresh_with(bundle(), cache.path(), &public);
        verify_with(bundle(), &asc, cache.path(), &no_fetch).unwrap();

        let mut revoked = public.clone();
        revoke_subkey(&signer, &mut revoked, RevocationCode::KeyRetired);
        refresh_with(bundle(), cache.path(), &revoked);
        let (result, _) = verify_refreshing(bundle(), &asc, cache.path(), &tarball(&[]));
        assert!(matches!(result, Err(Error::UnknownIssuer { .. })));
    }

    #[test]
    fn test_a_cached_subkey_the_trusted_key_did_not_bind_is_dropped() {
        let asc = refresh_fixture("signed-by-B-subkey.asc");
        let (mut forged, _) = SignedPublicKey::from_string(&refresh_fixture("bundle.asc")).unwrap();
        let (new_key, _) = SignedPublicKey::from_string(&refresh_fixture("B.asc")).unwrap();
        forged
            .public_subkeys
            .push(new_key.public_subkeys[0].clone());
        let cache = tempfile::tempdir().unwrap();
        std::fs::write(cache.path().join(CACHE_FILE), armored(&forged)).unwrap();
        let (result, calls) = verify_refreshing(test_bundle(), &asc, cache.path(), &tarball(&[]));
        assert_eq!(calls, 1);
        assert!(matches!(result, Err(Error::UnknownIssuer { .. })));
    }

    #[test]
    fn test_refreshed_subkeys_are_capped() {
        let (signer, public) = test_signer();
        let mut many = without_subkeys(&public);
        for seed in 0..=MAX_REFRESHED_SUBKEYS {
            let subkey = test_subkey(u8::try_from(seed).unwrap() + 100);
            let mut key = without_subkeys(&public);
            key.public_subkeys = vec![SignedPublicSubKey {
                key: subkey.public_key().clone(),
                signatures: vec![],
            }];
            let signer = TestSigner {
                primary: signer.primary.clone(),
                subkey,
            };
            rebind_subkey(&signer, &mut key, AT, Some(sign_only()), true);
            many.public_subkeys.extend(key.public_subkeys);
        }
        let mut keyring = keyring_of(without_subkeys(&public));
        keyring.add_refreshed(&[many]);
        assert_eq!(keyring.refreshed().count(), MAX_REFRESHED_SUBKEYS);

        // The cache holds them all, and reads back the same.
        let cache = tempfile::tempdir().unwrap();
        write_cache(cache.path(), "unknown", &keyring).unwrap();
        let cached = std::fs::read_to_string(cache.path().join(CACHE_FILE)).unwrap();
        let mut reloaded = keyring_of(without_subkeys(&public));
        reloaded.add_refreshed(&parse_keys(&cached));
        let fingerprints = |k: &Keyring| {
            k.refreshed()
                .map(KeyDetails::fingerprint)
                .collect::<Vec<_>>()
        };
        assert_eq!(fingerprints(&reloaded), fingerprints(&keyring));
    }

    #[test]
    fn test_user_keys_are_trusted_like_bundled_ones() {
        let (signer, public) = test_signer();
        let asc = clearsigned_by_subkey(&signer);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("user-keys.asc");
        std::fs::write(&path, armored(&public)).unwrap();
        let verified = verify(&asc, UNKNOWN_RELEASE, dir.path(), Some(&path), &no_fetch).unwrap();
        assert_eq!(verified.user_keys.as_deref(), Some(path.as_path()));
        assert_eq!(verified.signer, "Test Signer <signer@example.com>");
        let bundled = verify(
            &fixture("v20.0.0"),
            "v20.0.0",
            dir.path(),
            Some(&path),
            &no_fetch,
        )
        .unwrap();
        assert_eq!(bundled.user_keys, None);

        let mut revoked = public.clone();
        revoke_subkey(&signer, &mut revoked, RevocationCode::KeyCompromised);
        std::fs::write(&path, armored(&revoked)).unwrap();
        let error = verify(&asc, UNKNOWN_RELEASE, dir.path(), Some(&path), &no_fetch).unwrap_err();
        assert!(error.to_string().contains("revoked"), "{error}");

        // A refresh may add a new signing subkey to a user key too.
        std::fs::write(&path, armored(&without_subkeys(&public))).unwrap();
        let mut keyring = Keyring::bundled();
        keyring.add_user_keys(&path).unwrap();
        refresh_with(keyring, dir.path(), &public);
        let verified = verify(&asc, UNKNOWN_RELEASE, dir.path(), Some(&path), &no_fetch).unwrap();
        assert_eq!(verified.user_keys.as_deref(), Some(path.as_path()));
    }

    #[test]
    fn test_user_keys_that_cant_be_used_are_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let garbage = dir.path().join("garbage.asc");
        std::fs::write(
            &garbage,
            "-----BEGIN PGP PUBLIC KEY BLOCK-----\n\nnot a key\n-----END PGP PUBLIC KEY BLOCK-----\n",
        )
        .unwrap();
        for path in [dir.path().join("missing.asc"), garbage] {
            let error = verify(
                &fixture("v20.0.0"),
                "v20.0.0",
                dir.path(),
                Some(&path),
                &no_fetch,
            )
            .unwrap_err();
            assert!(matches!(error, Error::UserKeys { .. }), "{error}");
            assert!(
                error.to_string().contains(&path.display().to_string()),
                "{error}"
            );
        }
    }

    fn user_keys_file(dir: &Path, armored: &str) -> PathBuf {
        let path = dir.join("user-keys.asc");
        std::fs::write(&path, armored).unwrap();
        path
    }

    #[test]
    fn test_a_revocation_in_a_user_copy_of_a_trusted_key_applies() {
        let (secret, public) = test_signer();
        let asc = clearsigned_by_subkey(&secret);
        let mut revoked_subkey = public.clone();
        revoke_subkey(&secret, &mut revoked_subkey, RevocationCode::KeyCompromised);
        let mut revoked_primary = public.clone();
        let revocation = config(SignatureType::KeyRevocation, AT, &secret.primary)
            .sign_key(&secret.primary, &Password::empty(), &public.primary_key)
            .unwrap();
        revoked_primary
            .details
            .revocation_signatures
            .push(revocation);
        let dir = tempfile::tempdir().unwrap();
        for revoked in [revoked_subkey, revoked_primary] {
            // An unrevoked copy in the bundle, a revoked one in the user file.
            let path = user_keys_file(dir.path(), &armored(&revoked));
            let mut keyring = keyring_of(public.clone());
            keyring.add_user_keys(&path).unwrap();
            let error = verify_with(keyring, &asc, dir.path(), &no_fetch).unwrap_err();
            assert!(error.to_string().contains("revoked"), "{error}");

            // The same key twice in the user file.
            let both = format!("{}\n{}", armored(&public), armored(&revoked));
            let path = user_keys_file(dir.path(), &both);
            let error =
                verify(&asc, UNKNOWN_RELEASE, dir.path(), Some(&path), &no_fetch).unwrap_err();
            assert!(error.to_string().contains("revoked"), "{error}");
        }
    }

    #[test]
    fn test_every_key_in_a_user_key_block_is_read() {
        let (secret, public) = test_signer();
        let (other, _) = SignedPublicKey::from_string(&refresh_fixture("B.asc")).unwrap();
        let mut block = vec![];
        pgp::armor::write(
            &vec![other, public],
            pgp::armor::BlockType::PublicKey,
            &mut block,
            None,
            true,
        )
        .unwrap();
        let block = String::from_utf8(block).unwrap();
        assert_eq!(block.matches(PUBLIC_KEY_BEGIN).count(), 1);
        let dir = tempfile::tempdir().unwrap();
        let path = user_keys_file(dir.path(), &block);
        let asc = clearsigned_by_subkey(&secret);
        let verified = verify(&asc, UNKNOWN_RELEASE, dir.path(), Some(&path), &no_fetch).unwrap();
        assert_eq!(verified.signer, "Test Signer <signer@example.com>");
    }

    #[test]
    fn test_a_user_key_that_cant_be_read_is_an_error() {
        let (_, public) = test_signer();
        let valid = armored(&public);
        let truncated = &valid[..valid.len() / 2];
        let corrupted = valid.replacen('\n', "\n!", 3);
        let dir = tempfile::tempdir().unwrap();
        for second in [truncated, &corrupted] {
            let path = user_keys_file(dir.path(), &format!("{valid}\n{second}"));
            let error = verify(
                &fixture("v20.0.0"),
                "v20.0.0",
                dir.path(),
                Some(&path),
                &no_fetch,
            )
            .unwrap_err();
            assert!(matches!(error, Error::UserKeys { .. }), "{error}");
        }
    }

    #[test]
    fn test_the_user_keys_cant_be_the_cache() {
        let (_, public) = test_signer();
        let dir = tempfile::tempdir().unwrap();
        let cache = dir.path().join("release-keys");
        std::fs::create_dir_all(&cache).unwrap();
        let path = cache.join(CACHE_FILE);
        std::fs::write(&path, armored(&public)).unwrap();
        // Also through another spelling of the same path.
        let other = cache.join("..").join("release-keys").join(CACHE_FILE);
        for path in [path, other] {
            let error = verify(
                &fixture("v20.0.0"),
                "v20.0.0",
                &cache,
                Some(&path),
                &no_fetch,
            )
            .unwrap_err();
            assert!(error.to_string().contains("fnm's cache"), "{error}");
        }
    }

    fn signers() -> Vec<(&'static str, &'static str)> {
        data_lines(SIGNERS)
            .map(|line| line.split_once(' ').unwrap())
            .collect()
    }

    fn version_key(version: &str) -> Vec<u64> {
        version[1..]
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_a_release_in_the_table_needs_the_key_that_signed_it() {
        let cache = tempfile::tempdir().unwrap();
        verify(
            &fixture("v20.0.0"),
            "v20.0.0",
            cache.path(),
            None,
            &no_fetch,
        )
        .unwrap();
        // Signed by another active releaser, Antoine du Hamel.
        let error = verify(
            &fixture("v25.2.0"),
            "v20.0.0",
            cache.path(),
            None,
            &no_fetch,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "it is signed by 5BE8A3F6C8A5C01D106C0AD820B1A390B168D356, but v20.0.0 was signed by 890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4"
        );
    }

    #[test]
    fn test_a_release_not_in_the_table_needs_an_active_key() {
        let cache = tempfile::tempdir().unwrap();
        let verified = verify(
            &fixture("v25.2.0"),
            UNKNOWN_RELEASE,
            cache.path(),
            None,
            &no_fetch,
        )
        .unwrap();
        assert_eq!(
            verified.fingerprint,
            "5BE8A3F6C8A5C01D106C0AD820B1A390B168D356"
        );
        // The DSA-1024 key from 2014 vouches only for the releases it signed.
        let error = verify(
            &fixture("v0.10.30"),
            UNKNOWN_RELEASE,
            cache.path(),
            None,
            &no_fetch,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "it is signed by retired key 7937DFD2AB06298B2293C3187D33FF9D0246406D (Timothy J Fontaine (OFTC) <tjfontaine@oftc.net>); upgrade fnm"
        );
        verify(
            &fixture("v0.10.30"),
            "v0.10.30",
            cache.path(),
            None,
            &no_fetch,
        )
        .unwrap();
    }

    #[test]
    fn test_the_table_of_signers() {
        let bundled = Keyring::bundled();
        let signers = signers();
        assert!(signers.len() >= 730);
        for pair in signers.windows(2) {
            assert!(version_key(pair[0].0) < version_key(pair[1].0), "{pair:?}");
        }
        for (version, fingerprint) in &signers {
            assert!(
                bundled
                    .keys
                    .iter()
                    .any(|k| format!("{:X}", k.key.fingerprint()) == *fingerprint),
                "{version} {fingerprint}"
            );
        }
        for version in ["v0.10.30", "v6.0.0", "v20.0.0", "v24.12.0", "v25.2.0"] {
            let signer = signers.iter().find(|(v, _)| *v == version).unwrap().1;
            let verified = bundled.verify(&fixture(version)).unwrap();
            assert_eq!(verified.fingerprint, signer, "{version}");
        }
    }

    #[test]
    fn test_every_retired_key_signed_a_release_in_the_table() {
        // Chris Dickinson signed io.js releases only, not nodejs.org ones.
        const IO_JS_ONLY: &str = "9554F04D7259F04124DE6B476D5A82AC7E37093B";
        let keyring = Keyring::bundled();
        let signers = signers();
        let active: Vec<_> = data_lines(ACTIVE_KEYS).collect();
        assert_eq!(
            keyring.keys.iter().filter(|k| k.active).count(),
            active.len()
        );
        for trusted in keyring.keys.iter().filter(|k| !k.active) {
            let fingerprint = format!("{:X}", trusted.key.fingerprint());
            assert!(
                fingerprint == IO_JS_ONLY || signers.iter().any(|(_, f)| *f == fingerprint),
                "{fingerprint} ({}) signs no release in the table",
                trusted.name
            );
        }
    }

    #[test]
    fn test_a_refresh_adds_no_subkey_to_a_retired_key() {
        let (_, public) = test_signer();
        for active in [false, true] {
            let mut keyring = keyring_of(without_subkeys(&public));
            keyring.keys[0].active = active;
            keyring.add_refreshed(std::slice::from_ref(&public));
            assert_eq!(keyring.refreshed().count(), usize::from(active));
        }
    }
}

/// Run by `.ci/update-release-keys.sh`, over the network.
#[cfg(test)]
mod update {
    use super::*;
    use std::fmt::Write as _;

    /// Adds the releases that `release-keys/signers.txt` lacks, with the
    /// primary key their checked `SHASUMS256.txt.asc` is signed by.
    #[test]
    #[ignore = "fetches from nodejs.org and rewrites release-keys/signers.txt"]
    fn update_signers() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/release-keys/signers.txt");
        let table = std::fs::read_to_string(path).unwrap();
        let mut signers: Vec<(String, String)> = data_lines(&table)
            .filter_map(|line| line.split_once(' '))
            .map(|(version, fingerprint)| (version.to_string(), fingerprint.to_string()))
            .collect();
        let index: Vec<serde_json::Value> = crate::http::get("https://nodejs.org/dist/index.json")
            .unwrap()
            .json()
            .unwrap();
        let keyring = Keyring::bundled();
        for version in index
            .iter()
            .filter_map(|release| release["version"].as_str())
        {
            let unsigned = crate::version::Version::parse(version)
                .is_ok_and(|v| crate::download_verification::predates_signed_checksums(&v));
            if unsigned || signers.iter().any(|(known, _)| known == version) {
                continue;
            }
            let url = format!("https://nodejs.org/dist/{version}/SHASUMS256.txt.asc");
            let response = crate::http::get(&url).unwrap();
            if !response.status().is_success() {
                continue; // From before signed checksums.
            }
            match keyring.verify(&response.text().unwrap()) {
                Ok(verified) => signers.push((version.to_string(), verified.fingerprint)),
                Err(e) => eprintln!("warning: {version} is not added: {e}"),
            }
        }
        let number = |version: &str| -> Vec<u64> {
            let numbers = version.trim_start_matches('v').split('.');
            numbers.map(|n| n.parse().unwrap_or(0)).collect()
        };
        signers.sort_by_key(|(version, _)| number(version));

        let source = BUNDLED_KEYS
            .lines()
            .find_map(|line| line.strip_prefix("# Source commit: "))
            .unwrap_or("unknown");
        let date = chrono::Utc::now().format("%Y-%m-%d");
        let mut text = format!(
            "# The primary key that signed each Node.js release's SHASUMS256.txt.asc.\n\
             # Generated: {date}, from nodejs/release-keys@{source}\n\
             # Extended by .ci/update-release-keys.sh. Do not edit by hand.\n"
        );
        for (version, fingerprint) in &signers {
            writeln!(text, "{version} {fingerprint}").unwrap();
        }
        std::fs::write(path, text).unwrap();

        for trusted in keyring.keys.iter().filter(|k| !k.active) {
            let fingerprint = format!("{:X}", trusted.key.fingerprint());
            if !signers.iter().any(|(_, f)| *f == fingerprint) {
                eprintln!(
                    "warning: retired key {fingerprint} ({}) signs no release in the table",
                    trusted.name
                );
            }
        }
        println!("release-keys/signers.txt: {} releases", signers.len());
    }
}
