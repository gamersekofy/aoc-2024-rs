/* Instructions:
    We're keeping our eyes on a patrolling guard and our job is to make sure
    the historians evade detection while looking for the Chief.

    Our input is a map of the lab (patrolled area) with the guard represented
    as either <, ^, v, or >, depending on the direction she's facing. All obstacles
    on the map are represented by #.

    Guard's patrolling rules:

        while (true) {
            if (facingObstacle()) {
                turnRight()
            } else {
                moveForward()
            }
        }

    Our objective: "predict the path of the guard. how many distinct positions
    will the guard visit before leaving the mapped area?"
*/

pub struct Part1;

impl Part1 {
    fn eval_map(map: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval() {
        let map = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

        let actual_visited_positions = 41;
        let evaluated_visited_positions = Part1::eval_map(map);

        assert_eq!(actual_visited_positions, evaluated_visited_positions);
    }
}
