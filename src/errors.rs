use super::*;
use std::fmt::Debug;
#[derive(Debug)]
pub struct ShopCustomError(pub String);

impl ShopCustomError {
    pub fn get_custom_error<E: Debug>(e: E) -> Self {
        let error_string = format!("{e:#?}");
        error!("{error_string}");
    
        Self(error_string)
    }
}
impl fmt::Display for ShopCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "internal error occurred,please try again later")
    }
}
impl Error for ShopCustomError {}
#[derive(Error, Display, Debug)]
struct ShopResponseError(ShopCustomError);

impl ResponseError for ShopCustomError {}
impl From<Box<dyn Error>> for ShopCustomError {

    fn from(b: Box<dyn Error>) -> Self {
        Self(b.to_string())
    }
}
impl ResponseError for ShopResponseError {}

