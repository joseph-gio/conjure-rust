use std::sync::{Arc, OnceLock};

use rustls::crypto::CryptoProvider;

pub use rustls;

/// docs
pub fn ring_crypto_provider<'a>() -> &'a Arc<CryptoProvider> {
    static PROVIDER: OnceLock<Arc<CryptoProvider>> = OnceLock::new();

    PROVIDER.get_or_init(|| Arc::new(rustls::crypto::ring::default_provider()))
}
