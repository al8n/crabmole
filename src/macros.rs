#[cfg(feature = "derive")]
pub use crabmole_derive::constn;

/// Declare statics like Go.
///
/// # Examples
/// ## Declare statics
/// ```rust
/// use crabmole::staticn;
///
/// // Declare statics
/// staticn! {
///     u8 {
///         pub A = 1;
///         pub(crate) B = 2;
///         C = 3;
///     }
/// }
///
/// assert_eq!(A, 1);
/// assert_eq!(B, 2);
/// assert_eq!(C, 3);
/// ```
#[macro_export]
macro_rules! staticn {
    ($($ty: ty {$(
        $(#[$inner:ident $($args:tt)*])*
        $vis: vis $name:ident = $value:expr;
    )*}),* $(,)?) => {
        $(
            $(
                $vis static $name: $ty = $value;
            )*
        )*
    };
}
