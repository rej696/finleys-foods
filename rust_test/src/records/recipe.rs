use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub summary: String,
    created: i32
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub id: i64,
    pub position: i64,
    pub title: String,
    pub quantity: String
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Step {
    pub id: i64,
    pub position: i64,
    pub text: String
}

impl crate::utils::AsRoute for Recipe {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/recipes/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialRecipe {
    pub title: Option<String>,
    pub summary: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>
    // pub ingredient_position: [6, Option<i64>],
    // pub ingredient_title: [Option<String>],
    // pub ingredient_quantity: [Option<String>],
    // pub step_position: [Option<i64>],
    // pub step_text: [Option<String>]
}

impl PartialRecipe {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "update recipes (title, summary, created) values (
            coalesce($1, recipes.title),
            coalesce($2, recipes.summary),
            datetime('now')
            ) where id = $3",
        )
        .bind(&self.title)
        .bind(&self.summary)
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "insert into recipes (title, summary, created) values (
            $1, $2, datetime('now'))",
        )
        .bind(&self.title)
        .bind(&self.summary)
    }
}

impl Recipe {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM recipes")
    }

    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("SELECT last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM recipes WHERE id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query {
        sqlx::query("DELETE FROM articles WHERE id = ?").bind(id)
    }

    // Select list of ingredients based on recipe_ingredients_map orderd by position
    pub fn get_ingredients_by_id(id: i64) -> QueryAs<Ingredient> {
        sqlx::query_as(
            "SELECT * FROM ingredients WHERE (SELECT ingredient_id 
            FROM recipe_ingredients_map WHERE recipe_id = ?)
            ORDER BY position ASC"
        ).bind(id)
    }

    // Select list of steps based on recipe_steps_map ordered by position
    pub fn get_steps_by_id(id: i64) -> QueryAs<Step> {
        sqlx::query_as(
            "SELECT * FROM steps WHERE (SELECT step_id FROM
            recipe_steps_map WHERE recipe_id = ?)
            ORDER BY position ASC"
        ).bind(id)
    }
}
