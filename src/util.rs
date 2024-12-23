use anyhow::{Error, Result};
use std::borrow::Cow;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, SystemTime};

pub fn time(day: u8, part: u8, callback: impl FnOnce() -> Result<String>) -> Duration {
    let start = SystemTime::now();
    let str = callback().unwrap();
    let delta = SystemTime::now().duration_since(start).unwrap();
    println!("d{day}p{part}: {delta:?}\t{str}");
    delta
}

pub fn time_sol_helper<T: Solution>(count_parsing: bool) -> Result<Duration> {
    if count_parsing {
        let day = T::day();
        let default_input = T::default_input()?;
        let mut duration = Duration::ZERO;
        T::reset_global_state();
        duration += time(day, 1, || Ok(format!("{:?}", T::run_p1(&default_input)?)));
        T::reset_global_state();
        duration += time(day, 2, || Ok(format!("{:?}", T::run_p2(&default_input)?)));
        Ok(duration)
    } else {
        let day = T::day();
        let default_input = T::default_input()?;
        let input = T::parse(&default_input)?;
        let mut duration = Duration::ZERO;
        T::reset_global_state();
        duration += time(day, 1, || Ok(format!("{:?}", T::p1(input)?)));
        let input = T::parse(&default_input)?;
        T::reset_global_state();
        duration += time(day, 2, || Ok(format!("{:?}", T::p2(input)?)));
        Ok(duration)
    }
}

pub fn time_sol<T: Solution>(count_parsing: bool) -> Duration {
    time_sol_helper::<T>(count_parsing).unwrap()
}

macro_rules! generate_for_tuples {
    ($macro: ident) => {
        $macro!(T, U);
        $macro!(T, U, V);
        $macro!(T, U, V, W);
        $macro!(T, U, V, W, X);
        $macro!(T, U, V, W, X, Y);
        $macro!(T, U, V, W, X, Y, Z);
        $macro!(T, U, V, W, X, Y, Z, A);
        $macro!(T, U, V, W, X, Y, Z, A, B);
        $macro!(T, U, V, W, X, Y, Z, A, B, C);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D, E);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D, E, F);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D, E, F, G);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D, E, F, G, H);
        $macro!(T, U, V, W, X, Y, Z, A, B, C, D, E, F, G, H, I);
    };
}

macro_rules! generate_for_integers {
    ($macro: ident) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(usize);
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(isize);
    };
}

macro_rules! generate_for_unsigned {
    ($macro: ident) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(usize);
    };
}

macro_rules! generate_for_signed {
    ($macro: ident) => {
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(isize);
    };
}

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
    fn reset_global_state() {}
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
    #[allow(dead_code)]
    fn to_string(&self) -> String;
}

impl ToString for Vec<u8> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self).into()
    }
}

pub trait ToOption {
    type Item;

    fn to_option(&self) -> Option<Self::Item>;
}

impl<T: Clone> ToOption for Result<T> {
    type Item = T;

    fn to_option(&self) -> Option<Self::Item> {
        match self {
            Err(_) => None,
            Ok(x) => Some(x.clone()),
        }
    }
}

macro_rules! generate_tuple_to_option {
    ($($t: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($t: Copy),*> ToOption for ($(Option<$t>),*) {
            type Item = ($($t),*);

            fn to_option(&self) -> Option<Self::Item> {
                if let &($(Some($t)),*) = self {
                    Some(($($t),*))
                } else {
                    None
                }
            }
        }
    };
}

generate_for_tuples!(generate_tuple_to_option);

pub trait ToResult {
    type Item;

    #[allow(dead_code)]
    fn to_result(&self) -> Result<Self::Item>;
}

impl<T: Clone> ToResult for Option<T> {
    type Item = T;

    fn to_result(&self) -> Result<Self::Item> {
        self.clone().ok_or(Error::msg(""))
    }
}

macro_rules! generate_tuple_to_result {
    ($($t: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($t: Copy),*> ToResult for ($(Result<$t>),*) {
            type Item = ($($t),*);

            fn to_result(&self) -> Result<Self::Item> {
                if let &($(Ok($t)),*) = self {
                    Ok(($($t),*))
                } else {
                    Err(Error::msg("tuple of results failed"))
                }
            }
        }
    };
}

generate_for_tuples!(generate_tuple_to_result);

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

macro_rules! generate_integer_from_bytes {
    ($t: ident) => {
        impl FromBytes for $t {
            fn from_bytes(bytes: &[u8]) -> Self {
                bytes.iter().fold(0, |p, &c| p * 10 + (c - b'0') as $t)
            }
        }
    };
}

generate_for_integers!(generate_integer_from_bytes);

pub trait TrySearch
where
    Self: Sized,
{
    fn try_search(bytes: &[u8]) -> (Option<Self>, usize);

    fn try_search_with_start(bytes: &[u8], i: usize) -> (Option<Self>, usize) {
        let (value, j) = Self::try_search(&bytes[i..]);
        (value, i + j)
    }
}

macro_rules! generate_unsigned_try_search {
    ($t: ident) => {
        impl TrySearch for $t {
            fn try_search(bytes: &[u8]) -> (Option<Self>, usize) {
                for i in 0..bytes.len() {
                    let b = bytes[i];
                    if b >= b'0' && b <= b'9' {
                        let mut i = i;
                        let mut value: $t = 0;
                        while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
                            value = value * 10 + (bytes[i] - b'0') as $t;
                            i += 1;
                        }
                        return (Some(value), i);
                    }
                }
                (None, bytes.len())
            }
        }
    };
}

generate_for_unsigned!(generate_unsigned_try_search);

macro_rules! generate_signed_try_search {
    ($t: ident) => {
        impl TrySearch for $t {
            fn try_search(bytes: &[u8]) -> (Option<Self>, usize) {
                for i in 0..bytes.len() {
                    let b = bytes[i];
                    if b >= b'0' && b <= b'9' || b == b'-' {
                        let negative = b == b'-';
                        let mut i = i;
                        if negative {
                            i += 1;
                        }
                        let mut value: $t = 0;
                        while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
                            value = value * 10 + (bytes[i] - b'0') as $t;
                            i += 1;
                        }
                        return (Some(if negative { -value } else { value }), i);
                    }
                }
                (None, bytes.len())
            }
        }
    };
}

generate_for_signed!(generate_signed_try_search);
