pub mod recipes;
use crate::templates::home::*;

pub async fn home(_request: crate::Request) -> tide::Result {
    Ok(HomeTemplate::new("Tide").into())
}
