use std::collections::HashMap;
use crate::days::day_09::path::Path;
use crate::parse_input;

#[derive(Debug)]
pub struct Map {
    paths:HashMap<usize,HashMap<usize,u32>>
}

#[derive(Default)]
struct MapBuilder {
    id:usize,
    cities:HashMap<String,usize>,
    paths:HashMap<usize,HashMap<usize,u32>>
}

impl MapBuilder {
    pub(crate) fn build(&self) -> Map {
        Map{ paths:self.paths.clone()}
    }
}


impl MapBuilder {
    fn add_path(&mut self, path: &Path) {
        let city1 = path.city1().to_string();
        let city2 = path.city2().to_string();


        if !self.cities.contains_key(&city1) {
            self.cities.insert(city1.clone(), self.id);
            self.id += 1;
        }

        if !self.cities.contains_key(&city2) {
            self.cities.insert(city2.clone(), self.id);
            self.id += 1;
        }


        let city1 = *self.cities.get(&city1).unwrap();
        let city2 = *self.cities.get(&city2).unwrap();

        self.paths
            .entry(city1)
            .or_insert_with(HashMap::<usize, u32>::new)
            .insert(city2, path.distance());

        self.paths
            .entry(city2)
            .or_insert_with(HashMap::<usize, u32>::new)
            .insert(city1, path.distance());
    }
}

impl Map {
    pub fn parse(lines:&[String]) -> Map {
        let mut map = MapBuilder::default();
        lines.iter()
            .map(|l| parse_input!(l,Path))
            .fold(&mut map,
                  |acc,path| {
                      acc.add_path(&path);
                      acc
                  });
        map.build()
    }

    pub fn nb_cities(&self) -> usize {
        self.paths.len()
    }

    pub fn distance(&self,city1:usize, city2:usize) -> Option<u32> {
        self.paths.get(&city1)
            .and_then(|m| m.get(&city2))
            .copied()
    }

    pub fn connected_cities(&self, city:usize) -> Option<&HashMap<usize,u32>> {
        self.paths.get(&city)
    }

    pub fn paths(&self) -> &HashMap<usize, HashMap<usize, u32>> {
        &self.paths
    }
}