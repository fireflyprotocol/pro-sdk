use std::sync::Once;

static TLS_INIT: Once = Once::new();

#[cfg(not(any(feature = "tls-rustls", feature = "tls-native-tls")))]
compile_error!("one TLS feature must be enabled: `tls-native-tls` (default) or `tls-rustls`");

/// Initializes TLS runtime defaults once for the current process.
///
/// With `tls-native-tls` (default), this is a no-op.
/// With `tls-rustls`, installs the aws-lc-rs rustls provider.
///
/// # Panics
///
/// Panics if the `tls-rustls` feature is enabled and the aws-lc-rs
/// crypto provider fails to install.
pub fn ensure_tls_runtime() {
    TLS_INIT.call_once(|| {
        #[cfg(feature = "tls-rustls")]
        {
            if rustls::crypto::CryptoProvider::get_default().is_none() {
                rustls::crypto::aws_lc_rs::default_provider()
                    .install_default()
                    .expect("failed to install rustls aws-lc-rs provider");
            }
        }

        #[cfg(all(feature = "tls-native-tls", not(feature = "tls-rustls")))]
        {}
    });
}
