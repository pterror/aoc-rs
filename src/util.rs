use anyhow::Result;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub trait Solution {
    type Input;
    fn day() -> u8;
    fn default_input() -> Result<Vec<u8>>;
    fn parse(input: &Vec<u8>) -> Result<Self::Input>;
    fn p1(input: Self::Input) -> Result<impl Debug>;
    fn p2(input: Self::Input) -> Result<impl Debug>;
    fn run_p1(input: &Vec<u8>) -> Result<impl Debug> {
        let input = Self::parse(input)?;
        Self::p1(input)
    }
    fn run_p2(input: &Vec<u8>) -> Result<impl Debug> {
        let input = Self::parse(input)?;
        Self::p2(input)
    }
}

pub fn read_bytes_fn(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

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

pub trait BytesLines {
    type Line;
    fn lines(&self) -> impl Iterator<Item = Self::Line>;
}

impl BytesLines for Vec<u8> {
    type Line = Vec<u8>;

    fn lines(&self) -> impl Iterator<Item = Self::Line> {
        self.split(|&x| x == b'\n').map(|x| x.to_vec())
    }
}

pub trait ToStr {
    fn to_str(&self) -> Cow<'_, str>;
}

impl ToStr for Vec<u8> {
    fn to_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self)
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<u8> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self).into()
    }
}

pub trait ParseBytes {
    fn parse<T: FromBytes>(&self) -> T;
}

impl ParseBytes for [u8] {
    fn parse<T: FromBytes>(&self) -> T {
        <T as FromBytes>::from_bytes(self)
    }
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0'))
    }
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as u16)
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as u32)
    }
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as u64)
    }
}

impl FromBytes for usize {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as usize)
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as i8)
    }
}

impl FromBytes for i16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as i16)
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as i32)
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as i64)
    }
}

impl FromBytes for isize {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as isize)
    }
}
