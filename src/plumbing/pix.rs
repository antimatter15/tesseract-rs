extern crate leptonica_sys;
extern crate thiserror;

use self::leptonica_sys::{pixFreeData, pixRead, pixReadMem};
use self::thiserror::Error;
use std::convert::AsRef;
use std::ffi::CStr;

/// Wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
pub struct Pix(*mut leptonica_sys::Pix);

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            pixFreeData(self.0);
        }
    }
}

impl AsRef<*mut leptonica_sys::Pix> for Pix {
    fn as_ref(&self) -> &*mut leptonica_sys::Pix {
        &self.0
    }
}

#[derive(Debug, Error)]
#[error("Pix::read returned null")]
pub struct PixReadError();

#[derive(Debug, Error)]
#[error("Pix::read_mem returned null")]
pub struct PixReadMemError();

impl Pix {
    /// Wrapper for [`pixRead`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a84634846cbb5e01df667d6e9241dfc53)
    ///
    /// Read an image from a filename
    pub fn read(filename: &CStr) -> Result<Self, PixReadError> {
        let ptr = unsafe { pixRead(filename.as_ptr()) };
        if ptr.is_null() {
            Err(PixReadError {})
        } else {
            Ok(Self(ptr))
        }
    }

    /// Wrapper for [`pixReadMem`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a027a927dc3438192e3bdae8c219d7f6a)
    ///
    /// Read an image from memory
    pub fn read_mem(img: &[u8]) -> Result<Self, PixReadMemError> {
        let ptr = unsafe { pixReadMem(img.as_ptr(), img.len()) };
        if ptr.is_null() {
            Err(PixReadMemError {})
        } else {
            Ok(Self(ptr))
        }
    }
}

#[test]
fn read_error_test() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::ffi::CString::new("fail")?;
    assert!(Pix::read(&path).is_err());
    Ok(())
}

#[test]
fn read_mem_error_test() {
    assert!(Pix::read_mem(&[]).is_err());
}
