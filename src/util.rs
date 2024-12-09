use anyhow::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub trait Solution {
    type Input;
    fn day() -> u8;
    fn default_input() -> Result<String>;
    fn parse(input: &String) -> Result<Self::Input>;
    fn p1(input: Self::Input) -> Result<impl Debug>;
    fn p2(input: Self::Input) -> Result<impl Debug>;
    fn run_p1(input: &String) -> Result<impl Debug> {
        let input = Self::parse(input)?;
        Self::p1(input)
    }
    fn run_p2(input: &String) -> Result<impl Debug> {
        let input = Self::parse(input)?;
        Self::p2(input)
    }
}

pub fn read_string_fn(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

pub fn read_bytes_fn(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_lines_fn(path: &str) -> Result<Vec<String>> {
    Ok(read_string_fn(path)?.lines().map(|x| x.into()).collect())
}

pub fn read_byte_lines_fn(path: &str) -> Result<Vec<Vec<u8>>> {
    Ok(read_bytes_fn(path)?
        .split(|&x| x == b'\n')
        .map(|x| x.into())
        .collect())
}

#[cfg(feature = "include_str")]
macro_rules! read_string {
    ($path: literal) => {
        std::io::Result::Ok(String::from(include_str!(concat!("../../", $path))))
    };
}

#[cfg(not(feature = "include_str"))]
macro_rules! read_string {
    ($path: literal) => {
        crate::util::read_string_fn($path)
    };
}

pub(crate) use read_string;

#[cfg(feature = "include_str")]
macro_rules! read_bytes {
    ($path: literal) => {
        std::io::Result::Ok(String::from(include_bytes!(concat!("../../", $path))))
    };
}

#[cfg(not(feature = "include_str"))]
#[allow(unused_macros)]
macro_rules! read_bytes {
    ($path: literal) => {
        crate::util::read_bytes_fn($path)
    };
}

#[allow(unused_imports)]
pub(crate) use read_bytes;

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

#[cfg(feature = "include_str")]
macro_rules! read_byte_lines {
    ($path: literal) => {
        std::io::Result::Ok(
            include_bytes!(concat!("../../", $path))
                .split(|&x| x == b'\n')
                .collect::<Vec<Vec<u8>>>(),
        )
    };
}

#[cfg(not(feature = "include_str"))]
macro_rules! read_byte_lines {
    ($path: literal) => {
        crate::util::read_byte_lines_fn($path)
    };
}

pub(crate) use read_byte_lines;

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

pub trait IntoOk
where
    Self: Sized,
{
    fn ok(self) -> Result<Self> {
        Ok(self)
    }
}

impl<T> IntoOk for T {}
