use std::collections::HashSet;

pub struct Part1 {}
pub struct Part2 {}

/*
Notes:
    A small elf needs our help. Our puzzle input looks something like this:

        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX

    Our objective is to find all occurrence of the word XMAS. This word
    search allows words to be horizontal, vertical, diagonal, written
    backwards, or even overlapping other words. Here are possible ways
    XMAS might appear:

        ....XXMAS.
        .SAMXMS...
        ...S..A...
        ..A.A.MS.X
        XMASAMX.MM
        X.....XA.A
        S.S.S.S.SS
        .A.A.A.A.A
        ..M.M.M.MM
        .X.X.XMASX

    How many times does XMAS appear?
*/

impl Part1 {
    pub fn solve(input: &str) -> usize {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let rows = grid.len();
        if rows == 0 {
            return 0;
        }

        let cols = grid[0].len();
        let dirs = [
            (0, 1),   // right
            (1, 0),   // down
            (1, 1),   // down-right
            (1, -1),  // down-left
            (0, -1),  // left
            (-1, 0),  // up
            (-1, -1), // up-left
            (-1, 1),  // up-right
        ];

        let mut set = HashSet::new();
        for i in 0..rows {
            for j in 0..cols {
                for (dx, dy) in dirs.iter() {
                    let mut s = String::with_capacity(4);
                    let mut positions = Vec::with_capacity(4);
                    let mut x = i as i32;
                    let mut y = j as i32;
                    for _ in 0..4 {
                        if x < 0 || y < 0 || x >= rows as i32 || y >= cols as i32 {
                            break;
                        }
                        s.push(grid[x as usize][y as usize]);
                        positions.push((x as usize, y as usize));
                        x += dx;
                        y += dy;
                    }
                    if s.len() == 4 {
                        if s == "XMAS" || s == "SAMX" {
                            positions.sort();
                            let key = (positions[0], positions[1], positions[2], positions[3]);
                            set.insert(key);
                        }
                    }
                }
            }
        }
        set.len()
    }
}
