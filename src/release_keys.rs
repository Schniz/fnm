//! The Node.js release signing keys fnm trusts, and the check of a release's
//! `SHASUMS256.txt.asc` against them.

use log::debug;
use pgp::composed::{CleartextSignedMessage, Deserializable, SignedPublicKey, SignedPublicSubKey};
use pgp::packet::{KeyFlags, Signature, SignatureType, SubpacketData};
use pgp::types::{Fingerprint, KeyDetails, Tag};

/// A snapshot of <https://github.com/nodejs/release-keys>, made by `.ci/update-release-keys.sh`.
const BUNDLED_KEYS: &str = include_str!("../release-keys/keys.asc");

/// Node.js releases carry one signature; more than this is not worth hashing for.
const MAX_SIGNATURES: usize = 8;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("it is signed by key {issuer}, which is not a known Node.js release key")]
    UnknownIssuer { issuer: String },
    #[error("its signature is not valid: {reason}")]
    InvalidSignature { reason: String },
}

/// A release whose signature checked out.
#[derive(Debug)]
pub struct Verified {
    pub signer: String,
    pub fingerprint: String,
    /// The primary key or subkey that made the signature.
    pub signing_key: String,
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
}

pub struct Keyring {
    keys: Vec<TrustedKey>,
}

impl Keyring {
    pub fn bundled() -> Self {
        Self {
            keys: parse_keys(BUNDLED_KEYS)
                .into_iter()
                .map(|key| TrustedKey {
                    name: user_name(&key),
                    key,
                })
                .collect(),
        }
    }

    /// Checks the clearsigned `asc` and returns its signed text. Text outside
    /// the signed block fails the check rather than being ignored.
    pub fn verify(&self, asc: &str) -> Result<Verified, Error> {
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
            .map(|k| 1 + k.key.public_subkeys.len())
            .sum();
        let mut unknown_issuer = None;
        let mut failure = None;

        for signature in signatures.iter().take(MAX_SIGNATURES) {
            if !matches!(
                signature.typ(),
                Some(SignatureType::Binary | SignatureType::Text)
            ) {
                failure.get_or_insert_with(|| "it is not a document signature".to_string());
                continue;
            }
            let (candidates, unusable) = self.candidates(signature);
            if candidates.is_empty() {
                match unusable {
                    Some(key) => failure.get_or_insert_with(|| {
                        format!("it is signed by key {key}, which is revoked or may not sign")
                    }),
                    None => unknown_issuer.get_or_insert_with(|| issuer(signature)),
                };
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
                    return Ok(Verified {
                        signer: trusted.name.clone(),
                        fingerprint: format!("{:X}", trusted.key.fingerprint()),
                        signing_key: key.fingerprint(),
                        signed_text,
                    });
                }
            }
            if has_issuer(signature) {
                failure.get_or_insert_with(|| "it doesn't match the signed text".to_string());
            } else {
                unknown_issuer.get_or_insert_with(|| issuer(signature));
            }
        }

        match (failure, unknown_issuer) {
            (Some(reason), _) => Err(Error::InvalidSignature { reason }),
            (None, Some(issuer)) => Err(Error::UnknownIssuer { issuer }),
            (None, None) => Err(Error::InvalidSignature {
                reason: "it has no signature".to_string(),
            }),
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
            for subkey in &key.public_subkeys {
                if matches(subkey) {
                    if signing_binding(key, subkey, true).is_some()
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

fn armor_blocks(armored: &str) -> impl Iterator<Item = &str> {
    const BEGIN: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----";
    const END: &str = "-----END PGP PUBLIC KEY BLOCK-----";
    let mut rest = armored;
    std::iter::from_fn(move || {
        let start = rest.find(BEGIN)?;
        let end = rest[start..].find(END)? + start + END.len();
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
            other @ Error::InvalidSignature { .. } => panic!("{other}"),
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

    /// Signed with by the tests below, after the signer's keys were made.
    const TEXT: &str = "3f4b5c0a2b1e7d9c8a6f5e4d3c2b1a0f9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b  node-v99.0.0-linux-x64.tar.xz\r\n";
    const AT: u32 = 1_800_000_000;

    /// The secret keys of a test signer, so tests can make signatures of any kind.
    struct TestSigner {
        primary: SecretKey,
        subkey: SecretSubkey,
    }

    /// A key with a signing subkey, made from fixed seeds.
    fn test_signer() -> (TestSigner, SignedPublicKey) {
        let created = Timestamp::from_secs(AT - 1_000_000);
        let pair = |seed: u8| {
            let secret =
                ed25519::SecretKey::try_from_bytes([seed; 32], ed25519::Mode::EdDSALegacy).unwrap();
            let public = PubKeyInner::new(
                KeyVersion::V4,
                PublicKeyAlgorithm::EdDSALegacy,
                created,
                None,
                PublicParams::EdDSALegacy((&secret).into()),
            )
            .unwrap();
            let secret = PlainSecretParams::EdDSALegacy(eddsa_legacy::SecretKey::Ed25519(secret));
            (public, SecretParams::Plain(secret))
        };
        let (public, secret) = pair(1);
        let primary_public = PublicKey::from_inner(public).unwrap();
        let primary = SecretKey::new(primary_public.clone(), secret).unwrap();
        let (public, secret) = pair(2);
        let subkey_public = PublicSubkey::from_inner(public).unwrap();
        let subkey = SecretSubkey::new(subkey_public.clone(), secret).unwrap();
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
        rebind_subkey(&signer, &mut public, AT - 1000, sign_only(), true);
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
        let keys = keys.into_iter().map(|key| TrustedKey {
            name: user_name(&key),
            key,
        });
        Keyring {
            keys: keys.collect(),
        }
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
        flags: KeyFlags,
        backsig: bool,
    ) {
        let subkey = &public.public_subkeys[0].key;
        let mut config = config(SignatureType::SubkeyBinding, at, &signer.primary);
        config
            .hashed_subpackets
            .push(Subpacket::regular(SubpacketData::KeyFlags(flags)).unwrap());
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
        rebind_subkey(&secret, &mut public, AT, encrypt_only, true);
        rejection(&keyring_of(public), by_subkey(&secret, AT));

        // A signing subkey needs the subkey's own signature in its binding.
        let (secret, mut public) = test_signer();
        rebind_subkey(&secret, &mut public, AT, sign_only(), false);
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
}
