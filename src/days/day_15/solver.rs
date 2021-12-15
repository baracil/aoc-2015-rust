use crate::days::day_15::ingredients::{Ingredient, Recipe};

pub fn solve(ingredients:&[Ingredient]) -> u32 {
    let mut recipe:Recipe = Recipe::default();

    solve_inner(ingredients,&mut recipe, &|_| true)
}

pub fn solve_with_calories(ingredients:&[Ingredient]) -> u32 {
    let mut recipe:Recipe = Recipe::default();

    solve_inner(ingredients,&mut recipe, &|r| r.has_right_amount_of_calories())
}


fn solve_inner<F>(ingredients:&[Ingredient], recipe:&mut Recipe, constraint:&F) -> u32
where  F : Fn(&Recipe) -> bool
{

    let nb_ingredients = ingredients.len();

    if nb_ingredients == 1 {
        let quantity = recipe.nb_ingredients_left();
        recipe.add_ingredient(&ingredients[0],quantity);
        let result = if constraint(recipe) { recipe.score()} else {0};
        recipe.add_ingredient(&ingredients[0],-quantity);
        return result;
    }

    let mut max_score = 0;
    for i in 0..recipe.nb_ingredients_left() {
        recipe.add_ingredient(&ingredients[0],i);

        max_score = max_score.max(solve_inner(&ingredients[1..], recipe, constraint));

        recipe.add_ingredient(&ingredients[0],-i);
    }

    max_score
}
