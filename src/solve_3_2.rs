const N : usize = 1024;

type Grid = [[u32;N];N];
type Position = (usize, usize);
type Direction = (isize, isize);

fn mv(position : Position, direction : Direction) -> Position {
    let new_row = position.0 as isize + direction.0;
    let new_col = position.1 as isize + direction.1;
    (new_row as usize, new_col as usize)
}

fn write_grid(rows : &mut Grid, position : Position, value: u32) {
    rows[position.0][position.1] = value;
}

fn read_grid(rows : &Grid, position : Position) -> u32 {
    rows[position.0][position.1]
}

fn turn_left(direction : Direction) -> Direction {
    let (row, col) = direction;
    if row == -1 {
        (0, 1)
    } else if col == 1 {
        (1, 0)
    } else if row == 1 {
        (0, -1)
    } else {
        (-1, 0)
    }
}

fn sum_neighbors(grid: &Grid, position: Position) -> u32 {
    let mut accum : u32 = 0;
    for col in [-1, 0, 1].iter() {
        for row in [-1, 0, 1].iter() {
            if *row != 0 || *col != 0 {
                let neighbor_position = mv(position, (*row, *col));
                accum += read_grid(grid, neighbor_position);
            }
        }
    }
    accum
}

pub fn solve(input: u32) -> u32 {
    let mut grid : Grid = [[0; N]; N];
    let mut position = (N/2, N/2);
    let mut direction = (0, 1);

    write_grid(&mut grid, position, 1);

    loop {
        position = mv(position, direction);
        let sum = sum_neighbors(&grid, position);
        if sum > input {
            return sum
        }
        write_grid(&mut grid, position, sum);
        let to_left = turn_left(direction);
        let left = mv(position, to_left);
        if read_grid(&grid, left) == 0 {
            direction = to_left;
        }
    }
}
