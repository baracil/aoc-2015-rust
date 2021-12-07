use crate::days::day_06::command::Command;
use crate::days::day_06::grid::Grid;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day06_launch(part: Part) -> AOCResult<String> {
    let school = parse_input(false)?;
    match part {
        Part::Part1 => part1(&school),
        Part::Part2 => part2(&school)
    }
}
fn part1(commands:&Vec<Command>) -> AOCResult<String> {
    let mut grid = Grid::default();

    commands.iter().for_each(|c| grid.apply_command_part1(&c));

    Ok(grid.count_brightness().to_string())
}

fn part2(commands:&Vec<Command>) -> AOCResult<String> {
    let mut grid = Grid::default();

    commands.iter().for_each(|c| grid.apply_command_part2(&c));

    Ok(grid.count_brightness().to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<Command>> {
    Problem::factory(for_test)(6).read_input_as_mapped_lines(|s| s.parse::<Command>().expect(format!("Cannot parse '{}'",s).as_str()))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_06::command::Command;
    use crate::days::day_06::grid::Grid;
    use crate::days::day_06::main::parse_input;
    use crate::days::day_06::position::Position;
    use crate::days::day_06::rectangle::Rectangle;

    #[test]
    #[ignore]
    fn day06_part1_test()  {
        let mut grid = Grid::default();
        let command1 = "turn on 0,0 through 999,999".parse::<Command>().unwrap();
        let command2 = "turn off 0,0 through 99,99".parse::<Command>().unwrap();
        let command3 = "toggle 99,99 through 100,100".parse::<Command>().unwrap();
        grid.apply_command(&command1);
        grid.apply_command(&command2);
        grid.apply_command(&command3);

        let nb_on = grid.count_nb_turned_on();
        assert_eq!(nb_on, 999998);
    }

    #[test]
    fn day06_test_rect_indices()  {
        let rect = Rectangle::with_corners(Position::at(1,1),Position::at(2,2));


        rect.indices(10).for_each(|i| println!("{}",i))
    }

    #[test]
    #[ignore]
    fn day06_part2_test()  {
    }
}