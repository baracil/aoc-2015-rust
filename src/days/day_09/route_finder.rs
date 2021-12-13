use crate::days::day_09::map::Map;

pub struct RouteFinder {
    route_distance: u32,
    nb_to_visit: usize,
    visited: Vec<bool>,
    comp:fn(u32,u32) -> u32,
    shortcut:fn(u32,u32) -> bool,
}

impl RouteFinder {
    fn for_min(nb_cities: usize) -> Self {
        let visited = (0..nb_cities).map(|_| false).collect();

        RouteFinder { route_distance: u32::MAX, nb_to_visit: nb_cities, visited,
            comp:|i1,i2| i1.min(i2),
        shortcut:|i1,i2| i1>i2}
    }
    fn for_max(nb_cities: usize) -> Self {
        let visited = (0..nb_cities).map(|_| false).collect();

        RouteFinder { route_distance: u32::MIN, nb_to_visit: nb_cities, visited, comp:|i1,i2| i1.max(i2), shortcut:|_,_| false }
    }


    pub fn find_shortest_route(map: &Map) -> u32 {
        RouteFinder::find_route(map,RouteFinder::for_min)
    }

    pub fn find_longest_route(map: &Map) -> u32 {
        RouteFinder::find_route(map,RouteFinder::for_max)
    }

    pub fn find_route(map: &Map, factory:fn(usize) -> RouteFinder) -> u32 {
        let mut finder = factory(map.nb_cities());


        for path in map.paths() {
            finder.visited[*path.0] = true;
            finder.nb_to_visit -= 1;
            for next in path.1 {
                finder._find_route(*path.0, *next.0, 0, map);
            }
            finder.nb_to_visit += 1;
            finder.visited[*path.0] = false;
        }

        finder.route_distance
    }

    fn _find_route(&mut self, current: usize, next: usize, dist: u32, map: &Map) {
        let next_dist = dist + map.distance(current, next).unwrap();


        if (self.shortcut)(next_dist, self.route_distance) {
            return
        };

        self.visited[next] = true;
        self.nb_to_visit -= 1;


        if self.nb_to_visit == 0 {
            self.route_distance = (self.comp)(self.route_distance, next_dist);
        }

        if let Some(connections) = map.connected_cities(next) {
            for &city in connections.keys() {
                if self.visited[city] {
                    continue;
                }
                self._find_route(next, city, next_dist, map);
            }
        };

        self.nb_to_visit += 1;
        self.visited[next] = false;

    }
}

