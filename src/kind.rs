//! This module lists the twelve kinds of minecraft NBT type IDs in the enum
//! [`Kind`], see [minecraft Wiki] for more about the tag definitions.
//!
//! [minecraft Wiki]: https://minecraft.wiki/w/NBT_format

use core::error::Error;
use core::fmt::{Display, Formatter};
use core::{fmt, mem};

/// The twelve numeric type ID of NBTs.
///
/// This does not include *TAG_End* which is used to mark the end of compound
/// tags, since it does not represent any NBT value. To represent the marker
/// tag, use [`None`] of an [`Option<Kind>`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A signed 8-bit integer, ranging from -128 to 127 (inclusive).
    ///
    /// NBT has no boolean data type, but byte value 0 and `1` can be
    /// represented as `true`, `false`.
    #[doc(alias = "TAG_Byte")]
    Byte = 0x01,
    /// A signed 16-bit integer, ranging from -32,768 to 32,767 (inclusive).
    #[doc(alias = "TAG_Short")]
    Short = 0x02,
    /// A signed 32-bit integer, ranging from -2,147,483,648 and 2,147,483,647
    /// (inclusive).
    #[doc(alias = "TAG_Int")]
    Int = 0x03,
    /// A signed 64-bit integer, ranging from -9,223,372,036,854,775,808 to
    /// 9,223,372,036,854,775,807 (inclusive).
    #[doc(alias = "TAG_Long")]
    Long = 0x04,
    /// A 32-bit, single-precision floating-point number, ranging from -3.4E+38
    /// to +3.4E+38.
    #[doc(alias = "TAG_Float")]
    Float = 0x05,
    /// A 64-bit, double-precision floating-point number, ranging from -1.7E+308
    /// to +1.7E+308.
    #[doc(alias = "TAG_Double")]
    Double = 0x06,
    /// An ordered list of signed 8-bit integers.
    ///
    /// The payloads consist of a signed 32-bit integer `S`, then `S` number of
    /// signed 8-bit integers.
    #[doc(alias = "TAG_Byte_Array")]
    ByteArray = 0x07,
    /// A UTF-8 string, which has a size rather than being null-terminated.
    ///
    /// The payloads consist of an unsigned 16-bit integer `L`, then a UTF-8
    /// string resembled by `L` bytes.
    #[doc(alias = "TAG_String")]
    String = 0x08,
    /// An ordered list of tags of the same type.
    ///
    /// The payloads consist of a byte denoting the tag ID of the list's
    /// contents, followed by a signed 32-bit integer `S`, then `S` number of
    /// payloads that correspond to the given tag ID.
    #[doc(alias = "TAG_List")]
    List = 0x09,
    /// An unordered list of attribute-value pairs.
    ///
    /// The payloads are fully formed tags, then followed by *TAG_End*. And the
    /// first byte in each tag is the tag ID, followed by an unsigned 16-bit
    /// integer `L`, then the name as a string in UTF-8 format resembled by `L`
    /// bytes (no two tags may have the same name). Finally, depending on the
    /// type of the tag, the bytes that follow are part of that tag's payload.
    Compound = 0x0A,
    /// An ordered list of signed 32-bit integers.
    ///
    /// The payloads consist of a signed 32-bit integer `S`, then `S` number of
    /// signed 32-bit integers.
    #[doc(alias = "TAG_Int_Array")]
    IntArray = 0x0B,
    /// An ordered list of signed 64-bit integers.
    ///
    /// The payloads consist of a signed 32-bit integer `S`, then `S` number of
    /// signed 64-bit integers.
    #[doc(alias = "TAG_Long_Array")]
    LongArray = 0x0C,
}

impl Kind {
    /// Attempts to convert from `u8` to `Kind`, returning an error if the given
    /// value is out of range.
    ///
    /// This function has the same functionality with trait [`TryFrom`].
    ///
    /// # Errors
    ///
    /// This function returns an error if `kind` is not between `1` and `12`,
    /// i.e., `kind < 1 || kind > 12`.
    #[inline]
    pub const fn new(kind: u8) -> Result<Self, NbtKindError> {
        if 1 <= kind && kind <= 12 {
            Ok(unsafe { Kind::new_unchecked(kind) })
        } else {
            Err(NbtKindError(()))
        }
    }

    /// Converts from `u8` into `Kind` without checking the given value.
    ///
    /// # Safety
    ///
    /// This function is safe if and only if `kind` is between `1` and `12`
    /// (inclusive), i.e., `1 <= kind && kind <= 12`.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(kind: u8) -> Self {
        debug_assert!(1 <= kind && kind <= 12);
        // SAFETY: The conversion is safe:
        // 1. The enum `Kind` is marked `repr(u8)` and has the same memory
        //    layout with `u8`;
        // 2. The caller guarantees `kind` is between `1` and `12` (inclusive).
        //    Thus the converted value is always a valid `Kind`.
        unsafe { mem::transmute(kind) }
    }
}

impl TryFrom<u8> for Kind {
    type Error = NbtKindError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Just forward to `Kind::new`.
        Kind::new(value)
    }
}

/// An error that is returned when cannot convert an `u8` into `Kind`.
///
/// This would be returned by [`Kind::new`] or the equivalent function
/// [`TryFrom::try_from`].
#[derive(Debug)]
pub struct NbtKindError(pub(crate) ());

impl Display for NbtKindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("cannot convert from `u8` into `Kind`, value out of range")
    }
}

impl Error for NbtKindError {}
