use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn read_file_fn(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

#[cfg(feature = "include_str")]
macro_rules! read_file {
    ($path: literal) => {
        std::io::Result::Ok(String::from(include_str!(concat!("../../", $path))))
    };
}

#[cfg(not(feature = "include_str"))]
macro_rules! read_file {
    ($path: literal) => {
        crate::util::read_file_fn($path)
    };
}

pub(crate) use read_file;

pub fn read_lines_fn(path: &str) -> Result<Vec<String>> {
    Ok(read_file_fn(path)?.lines().map(|x| x.into()).collect())
}

#[cfg(feature = "include_str")]
macro_rules! read_lines {
    ($path: literal) => {
        std::io::Result::Ok(
            include_str!(concat!("../../", $path))
                .lines()
                .map(String::from)
                .collect::<Vec<String>>(),
        )
    };
}

#[cfg(not(feature = "include_str"))]
macro_rules! read_lines {
    ($path: literal) => {
        crate::util::read_lines_fn($path)
    };
}

pub(crate) use read_lines;

pub fn to<T: FromStr>(str: &str) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: 'static,
{
    Ok(str.parse()?)
}

pub trait CollectVec<T> {
    fn collect_vec(self) -> Vec<T>;
}

impl<T, U: Iterator<Item = T>> CollectVec<T> for U {
    fn collect_vec(self) -> Vec<T> {
        self.collect::<Vec<_>>()
    }
}

pub trait CollectResult<T> {
    fn collect_result(self) -> Result<Vec<T>>;
}

impl<T, U: Iterator<Item = Result<T>>> CollectResult<T> for U {
    fn collect_result(self) -> Result<Vec<T>> {
        self.collect::<Result<_>>()
    }
}
