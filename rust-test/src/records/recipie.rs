use serde::{Deserialize, Serialize};
type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Recipie {
    pub id: i64,
    pub title: String,
    pub summary: String,
    created: i32
}

impl crate::utils::AsRoute for Recipie {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/recipies/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialRecipie {
    pub title: Option<String>,
    pub summary: Option<String>,
}

impl PartialRecipie {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "update recipies (title, summary, created) values (
            coalesce($1, recipies.title),
            coalesce($2, recipies.summary),
            datetime('now')
            ) where id = $3",
        )
        .bind(&self.title)
        .bind(%self.summary)
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "insert into recipies (title, summary, created) values (
            $1, $2, datetime('now'))",
        )
        .bind(&self.title)
        .bind(&self.summary)
    }
}

impl Recipie {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("select * from recipies")
    }

    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("select last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("select * from recipies where id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query {
        sqlx::query("delete from articles where id = ?").bind(id)
    }

    pub fn get_ingredients_by_id(id: i64) -> QueryAs<Ingredient> {
        sqlx::query(
            "select * from ingredients where (select ingredient_id 
            from recipie_ingredients_map where recipie_id = ?",
        ).bind(id)
    }
}
