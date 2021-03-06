// Better math always trumps more code

fn rank(input: i32) -> i32 {
    ((((input as f64).sqrt() - 1.0) / 2.0) as i32) + 1
}

fn position(input: i32) -> (i32,i32) {
    if input == 1 {
        return (0, 0)
    }
    let r = rank(input);
    let bottom_right_corner = (2*r+1)*(2*r+1);
    let bottom_left_corner = bottom_right_corner - 2*r;
    let top_left_corner = bottom_left_corner - 2*r;
    let top_right_corner = top_left_corner - 2*r;

    let row;
    let col;
    if input < top_right_corner {
        row = top_right_corner - input - r;
        col = r;
    } else if input < top_left_corner {
        row = -1 * r;
        col = top_left_corner - input - r;
    } else if input < bottom_left_corner {
        row = bottom_left_corner - input - r;
        col = -1 * r;
    } else {
        row = r;
        col = bottom_right_corner - input - r;
    }
    (row, col)
}

pub fn solve(input: i32) -> i32 {
    let (row, col) = position(input);
    return row.abs() + col.abs();
}

#[test]
fn example1() {
    let answer = solve(1);
    assert_eq!(0, answer);
}

#[test]
fn example2() {
    let answer = solve(12);
    assert_eq!(3, answer);
}

#[test]
fn example3() {
    let answer = solve(23);
    assert_eq!(2, answer);
}

#[test]
fn example4() {
    let answer = solve(1024);
    assert_eq!(31, answer);
}

#[test]
fn test_rank() {
    let r = rank(55);
    assert_eq!(4, r);
}

#[test]
fn test_position1() {
    let (row, col) = position(29);
    assert_eq!(-1, row);
    assert_eq!(3, col);
}

#[test]
fn test_position2() {
    let (row, col) = position(64);
    assert_eq!(-4, row);
    assert_eq!(-3, col);
}
