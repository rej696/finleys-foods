use crate::records::{Recipe, PartialRecipe, Ingredient, Step};
use askama::Template;

#[derive(Template)]
#[template(path = "recipes/form.html")]
pub struct RecipeForm<'a> {
    title: &'a str,
    summary: &'a str,
    action: String,
}

impl<'a> RecipeForm<'a> {
    pub fn for_partial_recipe(recipe: &'a PartialRecipe) -> Self {
        Self {
            title: recipe.title.as_deref().unwrap_or_default(),
            summary: recipe.summary.as_deref().unwrap_or_default(),
            action: "/recipes".into(),
        }
    }
}

#[derive(Template)]
#[template(path = "recipes/index.html")]
pub struct IndexTemplate<'a> {
    recipes: &'a [Recipe],
}

impl<'a> IndexTemplate<'a> {
    pub fn for_recipes(recipes: &'a [Recipe]) -> Self {
        Self { recipes }
    }
}

#[derive(Template)]
#[template(path = "recipes/show.html")]
pub struct ShowTemplate<'a> {
    title: &'a str,
    summary: &'a str,
    ingredients: &'a [Ingredient],
    steps: &'a [Step],
}

impl<'a> ShowTemplate<'a> {
    pub fn for_recipe(
        recipe: &'a Recipe,
        ingredients: &'a [Ingredient],
        steps: &'a [Step]
    ) -> Self {
        Self {
            title: &recipe.title,
            summary: &recipe.summary,
            ingredients: &ingredients,
            steps: &steps,
        }
    }
}
