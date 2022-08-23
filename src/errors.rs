use super::*;
use std::fmt::Debug;
#[derive(Debug)]
pub struct ShopCustomError(pub String);

impl ShopCustomError {
    pub fn getCustomError<E: Debug>(e: E) -> ShopCustomError {
        return Self(format!("{e:#?}"));
    }
}
impl fmt::Display for ShopCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}
impl Error for ShopCustomError {}
#[derive(Error, Display, Debug)]
struct ShopResponseError(ShopCustomError);

impl ResponseError for ShopCustomError {}
impl ResponseError for ShopResponseError {}