use anyhow::anyhow;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
