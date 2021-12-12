use crate::days::day_13::graph::Graph;

pub struct SeatingFinder {
    happiness: i32,
    nb_to_seat: usize,
    visited: Vec<bool>,
}

impl SeatingFinder {


    pub fn find_happiest_seating(graph:&Graph) -> i32 {
        let visited = (0..graph.nb_guests()).map(|_| false).collect();

        let mut finder = SeatingFinder{happiness:0,nb_to_seat:graph.nb_guests(),visited};

        for guest in 0..graph.nb_guests() {

            finder.visited[guest] = true;
            finder.nb_to_seat -= 1;

            for neighbors in graph.get_neighbor(guest) {
                for (neighbor,_) in neighbors {
                    finder.find_happiest(guest,guest, *neighbor, 0, graph)
                }
            }

            finder.visited[guest] = false;
            finder.nb_to_seat += 1;
        };

        finder.happiness
    }

    fn find_happiest(&mut self, first:usize, current:usize, next:usize, current_happiness:i32, graph:&Graph ) {

        let new_happiness = current_happiness + graph.get_happiness(current,next);

        self.visited[next] = true;
        self.nb_to_seat -=1;

        if self.nb_to_seat == 0 {
            let final_happiness = new_happiness + graph.get_happiness(next,first);
            self.happiness = self.happiness.max(final_happiness);
        } else {
            for neighbors in graph.get_neighbor(next) {
                for (neighbor,_) in neighbors {
                    if !self.visited[*neighbor] {
                        self.find_happiest(first, next, *neighbor,new_happiness,graph)
                    }
                }
            }
        }

        self.visited[next] = false;
        self.nb_to_seat +=1;
    }

}