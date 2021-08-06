use crate::records::{Recipie, PartialRecipie, Ingredient, Step};
use askama::Template;

#[derive(Template)]
#[template(path = "recipies/form.html")]
pub struct RecipieForm<'a> {
    title: &'a str,
    summary: &'a str,
    action: String,
}

impl<'a> RecipieForm<'a> {
    pub fn for_partial_recipie(recpie: &'a PartialRecipie) -> Self {
        Self {
            title: recipie.title.as_deref().unwrap_or_default(),
            summary: recipie.summary.as_deref().unwrap_or_default(),
            action: "/recipies".into(),
        }
    }
}

#[derive(Template)]
#[template(path = "recipies/index.html")]
pub struct IndexTemplate<'a> {
    recipies: &'a [Recipie],
}

impl<'a> IndexTemplate<'a> {
    pub fn for_recipies(recipies: &'a [Recipie]) -> Self {
        Self { recipies }
    }
}

#[derive(Template)]
#[template(path = "recipies/show.html")]
pub struct ShowTemplate<'a> {
    title: &'a str,
    summary: &recipie.summary,
    ingredients: &'a [Ingredient],
    steps: &'a [Step],
}

impl<'a> ShowTemplate<'a> {
    pub fn for_recipie(recipie: &'a Recipie) -> Self {
        Self {
            title: &recipie.title,
            summary: &recipie.summary,
            ingredients: &recipie.ingredients,
            steps: &recipie.steps,
        }
    }
}