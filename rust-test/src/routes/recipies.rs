use crate::records::{Recipie, PartialRecipie};
use crate::templates::recipies::*;
use crate::utils;
use sqlx::prelude::*;

pub async fn index(request: crate::Request) -> tide::Result {
    let recipies = Recipie::all().fetch_all(&request.state().db).await?;
    Ok(IndexTemplate::for_recipies(recipies.as_slice()).into())
}

pub async fn show(request: crate::Request) -> tide::Result {
    let recipie = Recipie::find_by_id(request.param("recipie_id")?.parse()?)
        .fetch_one(&request.state().db)
        .await?;
    
    Ok(ShowTemplate::for_recipie(&recipie).into())
}

