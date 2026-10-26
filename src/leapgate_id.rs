use sha2::{Digest, Sha256};

/// Maps the upstream MAC-derived id to a different 9-digit id, so that
/// Leapgate and RustDesk on the same machine pointing at one server do not
/// register the same id.
pub(crate) fn salt(mac_id: u32) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(b"leapgate-auto-id\0");
    hasher.update(mac_id.to_le_bytes());
    let digest = hasher.finalize();
    let h = u32::from_le_bytes([digest[0], digest[1], digest[2], digest[3]]);
    (h & 0x0FFF_FFFF) | 0x1000_0000
}

#[cfg(test)]
mod tests {
    use super::salt;

    #[test]
    fn salted_id_is_nine_digits_and_differs_from_input() {
        for (mac_id, expected) in [
            (0, 501_480_547),
            (0x1234_5678, 394_683_129),
            (0x1FFF_FFFF, 510_268_549),
        ] {
            let id = salt(mac_id);
            assert_eq!(id, expected);
            assert_ne!(id, mac_id);
            assert_eq!(id.to_string().len(), 9);
        }
    }
}
