use std::str::FromStr;

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Default)]
pub struct Recipe {
    nb_ingredients: i32,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Recipe {
    pub fn nb_ingredients_left(&self) -> i32 {
        100 - self.nb_ingredients
    }

    pub fn has_right_amount_of_calories(&self) -> bool {
        self.calories == 500
    }
}

impl Recipe {
    pub(crate) fn score(&self) -> u32 {
        (self.capacity.max(0) * self.durability.max(0) * self.texture.max(0) * self.flavor.max(0))
            as u32
    }
}

impl Recipe {
    pub fn add_ingredient(&mut self, ingredient: &Ingredient, quantity: i32) {
        self.nb_ingredients += quantity;
        self.capacity += ingredient.capacity * quantity;
        self.durability += ingredient.durability * quantity;
        self.flavor += ingredient.flavor * quantity;
        self.texture += ingredient.texture * quantity;
        self.calories += ingredient.calories * quantity;
    }
}

impl FromStr for Ingredient {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = line.split(' ').collect();
        let capacity = parse_i32(tokens[2]);
        let durability = parse_i32(tokens[4]);
        let flavor = parse_i32(tokens[6]);
        let texture = parse_i32(tokens[8]);
        let calories = parse_i32(tokens[10]);

        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

fn parse_i32(token: &str) -> i32 {
    if token.ends_with(',') {
        return (token)[0..token.len() - 1].parse::<i32>().unwrap();
    }
    token.parse::<i32>().unwrap()
}
