use anyhow::anyhow;
use std::{ffi::{OsString, OsStr}};

pub fn u8_to_ascii(c: &[u8]) -> anyhow::Result<String> {
    let mut s = String::new();
    for i in c {
        if i.is_ascii() {
        let i = i.clone();
        s.push(i.into());
        } else {
        return Err(anyhow!("{} is a invaild ascii number", i));
        }
    }
    Ok(s)
}


pub fn u2w(u8str: &str) -> Vec<u16> {
    use std::os::windows::prelude::OsStrExt;
	OsStr::new(u8str).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

pub fn w2u(wstr: *const u16) -> String {
    use std::os::windows::prelude::OsStringExt;
    unsafe {
        let len = (0..std::isize::MAX).position(|i| *wstr.offset(i) == 0).unwrap();
        let slice = std::slice::from_raw_parts(wstr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
