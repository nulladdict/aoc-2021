use nalgebra::SMatrix;

const SIZE: usize = 5;
type Board = SMatrix<Option<usize>, SIZE, SIZE>;

fn main() {
    let input = include_str!("in");
    let (order, boards) = input.split_once("\n\n").unwrap();
    let order = order
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let boards = boards.split("\n\n").map(parse_board).collect::<Vec<_>>();

    dbg!(part1(&order, boards.clone()));
    dbg!(part2(&order, boards));
}

fn parse_board(board: &str) -> Board {
    Board::from_iterator(board.split_whitespace().map(|n| Some(n.parse().unwrap())))
}

fn part1(order: &[usize], mut boards: Vec<Board>) -> usize {
    for &number in order {
        for board in boards.iter_mut() {
            mark(board, number);
            if is_win(board) {
                return score(&board, number);
            }
        }
    }
    unreachable!()
}

fn part2(order: &[usize], mut boards: Vec<Board>) -> usize {
    for &number in order {
        if boards.len() == 1 {
            let board = &mut boards[0];
            mark(board, number);
            if is_win(board) {
                return score(board, number);
            }
            continue;
        }
        for board in boards.iter_mut() {
            mark(board, number);
        }
        boards.retain(|board| !is_win(board));
    }
    unreachable!()
}

fn mark(board: &mut Board, number: usize) -> () {
    for value in board.iter_mut() {
        if Some(number) == *value {
            value.take();
        }
    }
}

fn is_win(board: &Board) -> bool {
    let has_row = board.row_iter().any(|row| row.iter().all(|v| v.is_none()));
    let has_col = board
        .column_iter()
        .any(|col| col.iter().all(|v| v.is_none()));
    has_row || has_col
}

fn score(board: &Board, number: usize) -> usize {
    let sum = board.iter().filter_map(|x| x.as_ref()).sum::<usize>();
    sum * number
}
