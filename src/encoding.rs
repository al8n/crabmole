/// Go `encoding/ascii85` library
#[cfg(feature = "ascii85")]
#[cfg_attr(docsrs, doc(cfg(feature = "ascii85")))]
pub mod ascii85;

/// Go `encoding/binary` library
#[cfg(feature = "binary")]
#[cfg_attr(docsrs, doc(cfg(feature = "binary")))]
pub mod binary;

/// Go `encoding/base32` library
#[cfg(feature = "base32")]
#[cfg_attr(docsrs, doc(cfg(feature = "base32")))]
pub mod base32;

/// base58 library
#[cfg(feature = "base58")]
#[cfg_attr(docsrs, doc(cfg(feature = "base58")))]
pub mod base58;

/// Go `encoding/base64` library
#[cfg(feature = "base64")]
#[cfg_attr(docsrs, doc(cfg(feature = "base64")))]
pub mod base64;

/// Go `encoding/hex` library
#[cfg(feature = "hex")]
#[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
pub mod hex;

// /// Go encoding/pem library
// pub mod pem;
