//! Codec support for ASN.1 Types.

#![allow(dead_code)]
mod per;

#[doc(inline)]
pub use per::PerCodecData;

#[doc(inline)]
pub use per::PerCodecError;

#[doc(inline)]
pub use per::PerCodecErrorCause;

#[doc(inline)]
pub use per::UnknownOpenType;

#[doc(inline)]
pub use per::aper;

#[doc(inline)]
pub use per::uper;

// For now making this public, eventually when we have a proper sequence extensions support, this
// will not be required anyways.
pub use per::common::decode::decode_sequence_extensions_skip_bits;

#[doc(hidden)]
pub use per::common::decode::decode_sequence_extension_additions_bitmap;

#[doc(hidden)]
pub use per::common::encode::encode_sequence_extension_additions_bitmap;

// A library must not cap `log`'s compile-time level (nextgcore#1433).
// `release_max_level_*` on this crate's `log` dependency is unified into every
// consumer's build and compiles their `debug!`/`trace!` out of release builds.
// CI runs the tests with `--release`, which is where a cap would show.
#[cfg(test)]
mod log_level_tests {
    #[test]
    fn log_static_max_level_is_not_capped() {
        assert_eq!(log::STATIC_MAX_LEVEL, log::LevelFilter::Trace);
    }
}
