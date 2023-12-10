use itertools::Itertools;

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let rows = input.len();
    let cols = input[0].len();
    let mut grid: Vec<Vec<Option<Pipe>>> = vec![vec![None; cols]; rows];
    let rows = rows as i64;
    let cols = cols as i64;
    let mut start = (0, 0);
    for (i, line) in input.iter().enumerate() {
        for (j, _char) in line.chars().enumerate() {
            let row = i as i64;
            let col = j as i64;
            let ((r_l, c_l), (r_r, c_r)) = match _char {
                '|' => ((row - 1, col), (row + 1, col)),
                '-' => ((row, col - 1), (row, col + 1)),
                'L' => ((row - 1, col), (row, col + 1)),
                'J' => ((row, col - 1), (row - 1, col)),
                '7' => ((row, col - 1), (row + 1, col)),
                'F' => ((row, col + 1), (row + 1, col)),
                'S' => {
                    start = (i, j);
                    ((-1, -1), (-1, -1))
                },
                '.' => continue,
                _ => panic!("Invalid symbol in input: {_char}"),
            };
            let left = if r_l >= 0 && r_l < rows && c_l >= 0 && c_l < cols {
                Some((r_l as usize, c_l as usize))
            } else { None };
            let right = if r_r >= 0 && r_r < rows && c_r >= 0 && c_r < cols {
                Some((r_r as usize, c_r as usize))
            } else { None };
            grid[i][j] = Some(Pipe { left, right });
        }
    }

    let neighbors = vec![
        (start.0 as i64 - 1, start.1 as i64), 
        (start.0 as i64 + 1, start.1 as i64), 
        (start.0 as i64, start.1 as i64 - 1), 
        (start.0 as i64, start.1 as i64 + 1)
    ];
    let (mut left, mut right) = neighbors
        .iter()
        .filter_map(|(r, c)| 
            if *r >= 0 && *r < rows && *c >= 0 && *c < cols { 
                Some((*r as usize, *c as usize)) 
            } else { None }
        )
        .filter(|(r, c)| grid[*r][*c].is_some_and(|p| p.is_accessible_from(start.0, start.1)))
        .collect_tuple()
        .unwrap();
    let mut prev_left = start;
    let mut prev_right = start;
    let mut result = 1;
    while left != right {
        let left_pipe = grid[left.0][left.1].unwrap();
        let right_pipe = grid[right.0][right.1].unwrap();
        let temp = left;
        left = if left_pipe.left.unwrap() != prev_left {
            left_pipe.left.unwrap()
        } else { left_pipe.right.unwrap() };
        prev_left = temp;
        let temp = right;
        right = if right_pipe.left.unwrap() != prev_right {
            right_pipe.left.unwrap()
        } else { right_pipe.right.unwrap() };
        prev_right = temp;
        result += 1;
    }
    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    return String::new();
}

#[derive(Clone, Copy)]
struct Pipe {
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
}

impl Pipe {
    fn is_accessible_from(&self, r: usize, c: usize) -> bool {
        self.left == Some((r, c)) || self.right == Some((r, c))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("....."),
            String::from(".S-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1_basic_extras() {
        let input = vec![
            String::from("-L|F7"),
            String::from("7S-7|"),
            String::from("L|7||"),
            String::from("-L-J|"),
            String::from("L|-JF"),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1_complex() {
        let input = vec![
            String::from("..F7."),
            String::from(".FJ|."),
            String::from("SJ.L7"),
            String::from("|F--J"),
            String::from("LJ..."),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "8");
    }

    #[test]
    fn test_part1_complex_extras() {
        let input = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "8");
    }

    
}