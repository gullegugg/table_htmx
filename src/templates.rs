use askama::Template;

use crate::api::Recipe;

#[derive(Template)]
#[template(path = "recipes.html")]
pub struct RecipesTemplate {
    recipes: Vec<Recipe>,
    page_number: u32,
}

impl RecipesTemplate {
    pub fn new(recipes: Vec<Recipe>, page_number: u32) -> Self {
        RecipesTemplate {
            recipes,
            page_number,
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    recipes: Vec<Recipe>,
    page_number: u32,
}

impl IndexTemplate {
    pub fn new(recipes: Vec<Recipe>, page_number: u32) -> Self {
        IndexTemplate {
            recipes,
            page_number,
        }
    }
}
