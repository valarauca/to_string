//! Note the conversions from OSString/OSStr will convert
//! invalid utf-8 characters into the [`U+FFFD REPLACEMENT CHARACTER`](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)
//! if they cannot convert.

use std::{
    borrow::Cow,
    ffi::{OsStr,OsString},
};

/// Converts _something_ from the Rust standard library into
/// a string.
///
/// Generally this involves cloning what ever it is passed
/// unless a comment notes that is is a special case.
pub trait IntoString {
    fn into_string(self) -> String;
}

impl<'a> IntoString for Cow<'a,OsStr> {
    /// Special case.
    ///
    /// This will inspect cow to see if the interior buffer is
    /// owned and perform a similiar inspect to `OsString::into_string`.
    fn into_string(self) -> String {
        match self {
            Cow::Owned(x) => {
                <OsString as IntoString>::into_string(x)
            }
            Cow::Borrowed(x) => x.into_string()
        }
    }
}
impl<'a> IntoString for &Cow<'a,OsStr> {
    fn into_string(self) -> String {
        self.to_string_lossy().to_string()
    }
}
impl<'a> IntoString for &&Cow<'a,OsStr> {
    fn into_string(self) -> String {
        self.to_string_lossy().to_string()
    }
}
impl<'a> IntoString for &&&Cow<'a,OsStr> {
    fn into_string(self) -> String {
        self.to_string_lossy().to_string()
    }
}
impl<'a> IntoString for &&&&Cow<'a,OsStr> {
    fn into_string(self) -> String {
        self.to_string_lossy().to_string()
    }
}
impl<'a> IntoString for &&&&&Cow<'a,OsStr> {
    fn into_string(self) -> String {
        self.to_string_lossy().to_string()
    }
}


impl IntoString for OsString {
    /// Special Case
    ///
    /// This call will first attempt
    /// [`into_string`](https://doc.rust-lang.org/std/ffi/struct.OsString.html#method.into_string)
    /// which will perserve ownership, falling back on the standard `to_string_lossy` method if
    /// that fails.
    ///
    /// That is to say it will optimistically attempt to transfer ownership without cloning
    /// the buffer.
    fn into_string(self) -> String {
        match OsString::into_string(self) {
            Ok(x) => x,
            Err(e) => {
                e.to_string_lossy().into_string()
            }
        }
    }
}
impl<'a> IntoString for &'a OsString {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&'a OsString {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&'a OsString {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&&'a OsString {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&&&'a OsString {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}

impl<'a> IntoString for &'a OsStr {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&'a OsStr {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&'a OsStr {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&&'a OsStr {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}
impl<'a> IntoString for &&&&&'a OsStr {
    fn into_string(self) -> String {
        self.to_string_lossy().into_string()
    }
}

impl<'a> IntoString for Cow<'a,str> {
    /// Special case.
    ///
    /// This will inspect cow to see if the interior buffer is
    /// owned and will avoid copying that case.
    fn into_string(self) -> String {
        match self {
            Cow::Owned(x) => x,
            Cow::Borrowed(x) => x.to_string(),
        }
    }
}
impl<'a> IntoString for &Cow<'a,str> {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&Cow<'a,str> {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&Cow<'a,str> {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&&Cow<'a,str> {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&&&Cow<'a,str> {
    fn into_string(self) -> String {
        self.to_string()
    }
}

impl<'a> IntoString for &'a str {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&'a str {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&'a str {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&&'a str {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl<'a> IntoString for &&&&&'a str {
    fn into_string(self) -> String {
        self.to_string()
    }
}

impl IntoString for String {
    /// Special case, absolutely nothing is done
    fn into_string(self) -> String {
        self
    }
}
impl IntoString for &String {
    fn into_string(self) -> String {
        self.clone()
    }
}
impl IntoString for &&String {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl IntoString for &&&String {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl IntoString for &&&&String {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl IntoString for &&&&&String {
    fn into_string(self) -> String {
        self.to_string()
    }
}

