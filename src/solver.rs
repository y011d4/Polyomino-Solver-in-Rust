use env::Board;
use env::Piece;

pub fn solve(board: &mut Board, pieces: &mut Vec<Piece>, depth: usize, cnt: &mut usize) -> bool {
    let emp_idx = get_first_emp_idx(&board);
    for i in 0..pieces.len() {
        if !pieces[i].used {
            for blocks in pieces[i].pos.clone().iter() {
                let mut is_prohibited = false;
                for block in blocks.iter() {
                    if emp_idx+block>=board.field.len() {
                        is_prohibited = true;
                    } else {
                        let tmp: bool = if board.field[emp_idx+block]!=0 { true } else { false };
                        is_prohibited = is_prohibited || tmp;
                    }
                }
                if is_prohibited {
                    continue
                }

                for block in blocks.iter() {
                    board.field[emp_idx+block] = i as isize + 1;
                }
                pieces[i].used = true;

                if depth == pieces.len()-1 {
                    *cnt += 1;
                    println!("{}", cnt);
                    println!("{}", board);
                    for block in blocks.iter() {
                        board.field[emp_idx+block] = 0;
                    }
                    pieces[i].used = false;
                    return true;
                }
                solve(board, pieces, depth+1, cnt);
                for block in blocks.iter() {
                    board.field[emp_idx+block] = 0;
                }
                pieces[i].used = false;
            }
        }
    }
    false
}

fn get_first_emp_idx(board: &Board) -> usize {
    let mut emp_idx: usize = board.field.len();
    for (i, elem) in board.field.iter().enumerate() {
        if *elem==0 {
            emp_idx = i;
            break;
        }
    }
    return emp_idx;
}
