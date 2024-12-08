use std::fs;

pub fn solve_a() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let matrix: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();

    let hits = count_hits("XMAS", &matrix);

    println!("{}", hits);
}

pub fn solve_b() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let matrix: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut hits = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if try_find_x_mas(&matrix, (x, y)) {
                hits += 1;
            }
        }
    }

    println!("{}", hits);
}

fn count_hits(word: &str, matrix: &Vec<Vec<char>>) -> usize {
    let mut hits = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            hits += try_find(word, &matrix, (x.try_into().unwrap(), y.try_into().unwrap()));
        }
    }

    hits
}

fn try_find(word: &str, matrix: &Vec<Vec<char>>, (x, y): (i32, i32)) -> usize {
    let directions = vec![
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ];
    directions.iter()
        .filter(|direction| try_find_direction(word, direction, matrix, (x, y)))
        .count()
}

fn try_find_direction(word: &str, direction: &Direction, matrix: &Vec<Vec<char>>, (x, y): (i32, i32)) -> bool {
    let (mut current_x, mut current_y) = (x, y);
    for search_char in word.chars() {
        if current_y < 0 || current_x < 0 {
            return false;
        }
        let current_x_index: usize = current_x.try_into().unwrap();
        let current_y_index: usize = current_y.try_into().unwrap();
        if current_y_index >= matrix.len() || current_x_index >= matrix[current_y_index].len() {
            return false;
        }

        let current_char = matrix[current_y_index][current_x_index];

        if search_char != current_char {
            return false;
        }
        
        (current_x, current_y) = get_next_position((current_x, current_y), direction);
    }
    true
}

fn try_find_x_mas(matrix: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    if y < 1 || y >= matrix.len() - 1 {
        return false;
    }

    if x < 1 || x >= matrix[y].len() - 1 {
        return false;
    }

    if matrix[y][x] != 'A' {
        return false;
    }

    if !((matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S') || (matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M')) {
        return false;
    }

    if !((matrix[y + 1][x - 1] == 'M' && matrix[y - 1][x + 1] == 'S') || (matrix[y + 1][x - 1] == 'S' && matrix[y - 1][x + 1] == 'M')) {
        return false;
    }

    true
}

fn get_next_position((x, y): (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::N => (x, y - 1),
        Direction::NE => (x + 1, y - 1),
        Direction::E => (x + 1, y),
        Direction::SE => (x + 1, y + 1),
        Direction::S => (x, y + 1),
        Direction::SW => (x - 1, y + 1),
        Direction::W => (x - 1, y),
        Direction::NW => (x - 1, y - 1),
    }
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_try_find_direction() {
        let matrix = vec![
            vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'],
            vec!['S', 'A', 'X', 'X', 'M', 'A', 'A'],
            vec!['S', 'A', 'M', 'M', 'M', 'A', 'M'],
            vec!['S', 'A', 'A', 'X', 'A', 'A', 'X'],
            vec!['S', 'A', 'S', 'X', 'M', 'S', 'X'],
        ];
        assert_eq!(try_find_direction("XMAS", &Direction::N, &matrix, (6, 3)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::N, &matrix, (6, 0)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::NE, &matrix, (3, 3)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::NE, &matrix, (3, 2)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::E, &matrix, (3, 0)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::E, &matrix, (2, 0)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::SE, &matrix, (2, 1)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::SE, &matrix, (3, 1)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::S, &matrix, (2, 1)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::S, &matrix, (3, 4)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::SW, &matrix, (3, 1)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::SW, &matrix, (4, 0)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::W, &matrix, (3, 0)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::W, &matrix, (0, 0)), false);
        assert_eq!(try_find_direction("XMAS", &Direction::NW, &matrix, (3, 3)), true);
        assert_eq!(try_find_direction("XMAS", &Direction::NW, &matrix, (8, 8)), false);
    }

    #[test]
    fn test_count_hits() {
        let matrix = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(count_hits("XMAS", &matrix), 18);
    }
}
