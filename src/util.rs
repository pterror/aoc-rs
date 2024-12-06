use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    Ok(read_file(path)?.lines().map(|x| x.into()).collect())
}

pub fn to<T: FromStr>(str: &str) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: 'static,
{
    Ok(str.parse()?)
}
