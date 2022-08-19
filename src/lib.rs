//! Note the conversions from OSString/OSStr will convert
//! invalid utf-8 characters into the [`U+FFFD REPLACEMENT CHARACTER`](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)
//! if they cannot convert.

use std::{
    borrow::Cow,
    ffi::{OsStr,OsString,CStr,CString},
};

/// Converts _something_ from the Rust standard library into
/// a string.
///
/// Generally this involves cloning what ever it is passed
/// unless a comment notes that is is a special case.
///
/// When working with types that cannot be trusted to be
/// converted to UTF-8 safely. The interface will replace
/// bad characters with the `U+FFFD` replacement character
///
/// In the event the entire buffer is NOT utf8, it will return
/// a buffer full of U+FFFD characters.
pub trait IntoString {
    fn into_string(self) -> String;
}


impl<'a> IntoString for Cow<'a,CStr> {
    // will attempt to gracefully transfer ownership
    fn into_string(self) -> String {
        match self {
            Cow::Owned(x) => <CString as IntoString>::into_string(x),
            Cow::Borrowed(x) => {
                local_to_str(x.to_bytes())
            }
        }
    }
}
impl<'a> IntoString for &Cow<'a,CStr> {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&Cow<'a,CStr> {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&Cow<'a,CStr> {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&&Cow<'a,CStr> {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}

impl IntoString for CString {
    /// Special Case
    ///
    /// Will attempt to check & not-reallocate the buffer if possible
    fn into_string(self) -> String {
        match CString::into_string(self) {
            Ok(x) => x,
            Err(e) => {
                local_to_str(e.into_cstring().as_bytes())
            }
        }
    }
}
impl<'a> IntoString for &'a CString {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&'a CString {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&'a CString {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&&'a CString {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&&&'a CString {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}

impl<'a> IntoString for &'a CStr {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&'a CStr {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&'a CStr {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&&'a CStr {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
}
impl<'a> IntoString for &&&&&'a CStr {
    fn into_string(self) -> String {
        local_to_str(self.to_bytes())
    }
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


fn local_to_str(x: &[u8]) -> String {
    match std::str::from_utf8(x) {
        Ok(x) => x.to_string(),
        Err(_) => {
            let mut s = String::with_capacity(x.len());
            s.extend((0..x.len()).map(|_| -> char { '\u{FFFD}' }));
            s
        }
    }
}
