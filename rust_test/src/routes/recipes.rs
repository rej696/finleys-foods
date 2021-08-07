use crate::records::{Recipe, PartialRecipe, Ingredient, Step};
use crate::templates::recipes::*;
use crate::utils;
use sqlx::prelude::*;

pub async fn index(request: crate::Request) -> tide::Result {
    let recipes = Recipe::all().fetch_all(&request.state().db).await?;
    Ok(IndexTemplate::for_recipes(recipes.as_slice()).into())
}

pub async fn show(request: crate::Request) -> tide::Result {
    let recipe = Recipe::find_by_id(request.param("recipe_id")?.parse()?)
        .fetch_one(&request.state().db)
        .await?;
    let ingredients = Recipe::get_ingredients_by_id(request.param("recipe_id")?.parse()?)
        .fetch_all(&request.state().db)
        .await?;
    let steps = Recipe::get_steps_by_id(request.param("recipe_id")?.parse()?)
        .fetch_all(&request.state().db)
        .await?;

    Ok(ShowTemplate::for_recipe(&recipe, &ingredients, &steps).into())
}

pub async fn delete(request: crate::Request) -> tide::Result {
    Recipe::delete_by_id(request.param("recipe_id")?.parse()?)
        .execute(&request.state().db)
        .await?;
    
    Ok(tide::Redirect::new("/").into())
}

pub async fn update(mut request: crate::Request) -> tide::Result {
    let recipe: PartialRecipe = utils::deserialize_body(&mut request).await?;
    let recipe_id = request.param("recipe_id")?.parse()?;
    let rows_updated = recipe
        .update_by_id(recipe_id)
        .execute(&request.state().db)
        .await?;
    
    if rows_updated == 1 {
        Ok(tide::Redirect::new(format!("/recipe/{}", recipe_id)).into())
    } else {
        Ok(RecipeForm::for_partial_recipe(&recipe).into())
    }
}

pub async fn create(mut request: crate::Request) -> tide::Result {
    let db = &request.state().db;
    let mut tx = db.begin().await?;
    let recipe : PartialRecipe = utils::deserialize_body(&mut request).await?;
    let created = recipe.create().execute(&mut tx).await?;

    if created == 1 {
        let (last_id,) = Recipe::last_id().fetch_one(&mut tx).await?;
        tx.commit().await?;

        Ok(tide::Redirect::new(format!("/recipe/{}", last_id)).into())
    } else {
        Ok(RecipeForm::for_partial_recipe(&recipe).into())
    }
}

pub async fn new(_request: crate::Request) -> tide::Result {
    let recipe = PartialRecipe::default();
    Ok(RecipeForm::for_partial_recipe(&recpie).into())
}