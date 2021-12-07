use crate::days::day_06::position::Position;

#[derive(Debug)]
pub struct Rectangle {
    left_up:Position,
    right_down:Position,
}

impl Rectangle {
    pub fn with_corners(corner1:Position, corner2:Position) -> Self {
        let left = corner1.x().min(corner2.x());
        let right = corner1.x().max(corner2.x());
        let up = corner1.y().min(corner2.y());
        let down = corner1.y().max(corner2.y());

        Rectangle{left_up:Position::at(left,up), right_down:Position::at(right,down)}
    }


    pub fn indices(&self, sliding:usize) -> impl Iterator<Item=usize> + '_ {
        let left_up = self.left_up().to_owned();



        (0..self.range_y())
            .map(move |r| (left_up.y()+r)*sliding )
            .flat_map(move |r| (0..self.range_x()).map(move |c| r+left_up.x()+c))
    }

    pub fn left_up(&self) -> &Position {
        &self.left_up
    }

    #[allow(dead_code)]
    pub fn right_down(&self) -> &Position {
        &self.right_down
    }

    pub fn range_x(&self) -> usize {
        self.right_down.x() - self.left_up.x()+1
    }

    pub fn range_y(&self) -> usize {
        self.right_down.y() - self.left_up.y()+1
    }
}