use std::io::BufRead;

use tokio::task::JoinSet;



fn assert(cond: bool) {
	if !cond {
		panic!("badness");
	}
}

type Board = [[usize; 9]; 9];

const ASCII_ZERO: usize = '0' as usize;
const ASCII_NL: char = '\n';

fn print_board(board: &Board) {
	let mut str = String::new();
    for i in 0..9 {
        for j in 0..9 {
            str.push(char::from_u32((board[i][j] + ASCII_ZERO) as u32).unwrap());
			//str = append(str, byte(board[i][j]) + ascii_zero);
		}
        str.push(ASCII_NL);
		//str = append(str, ascii_nl);
	}
	print!("{}", str);
}

fn partial_verify(board: &Board, x: usize, y: usize) -> bool {
	let base_x = (x / 3) * 3;
	let base_y = (y / 3) * 3;
	for i in 0..9 {
		if (i != y) && (board[x][i] == board[x][y]) {
			return false;
		}
		if (i != x) && (board[i][y] == board[x][y]) {
			return false;
		}
		let pos_x = base_x + (i / 3);
		let pos_y = base_y + (i % 3);
		if (pos_x != x || pos_y != y) && (board[pos_x][pos_y] == board[x][y]) {
			return false;
		}
	}
	return true;
}

fn solve(board: &mut Board, x: usize, y: usize) -> bool {
	let z = (x * 9) + y + 1;
	if z == 82 {
		return true;
	}
	if board[x][y] != 0 {
		return solve(board, z / 9, z % 9);
	}
	for i in 1..=9 {
		board[x][y] = i;
		if partial_verify(board, x, y) {
			if solve(board, z / 9, z % 9) {
				return true;
			}
		}
	}
	board[x][y] = 0;
	return false;
}

fn verify(board: &Board) -> bool {
	for i in 0..9 {
		//var row_check [10]bool
        let mut row_check = [false; 10];
		//var col_check [10]bool
        let mut col_check = [false; 10];
		for j in 0..9 {
			if board[i][j] == 0 {
				return false;
			}
			if row_check[board[i][j]] {
				return false;
			}
			row_check[board[i][j]] = true;
			if col_check[board[i][j]] {
				return false;
			}
			col_check[board[i][j]] = true;
		}
	}
	//for i := 0; i < 9; i += 3 {
	for i in (0..9).step_by(3) {
    	//for j := 0; j < 9; j += 3 {
        for j in (0..9).step_by(3) {
			//var check [10]bool
            let mut check = [false; 10];
			//for k := 0; k < 9; k++ {
            for k in 0..9 {
				let x = i + (k / 3);
				let y = j + (k % 3);
				if check[board[x][y]] {
					return false
				}
				check[board[x][y]] = true;
			}
		}
	}
	return true
}

fn read_line(line: Vec<char>, board: &mut Board) {
	let ascii_zero = '0';
	let ascii_dot = '.';
	for i in 0..9 {
		for j in 0..9 {
			let mut ch= line[(i * 9) + j];
			if ch == ascii_dot {
				ch = ascii_zero;
			}
			board[i][j] = ch as usize - ASCII_ZERO as usize;
		}
	}
}

async fn process(path: String) {
    let lines = std::io::BufReader::new(std::fs::File::open(path).unwrap()).lines().flatten();
	//file, err := os.Open(path)
	//defer file.Close()
	//var scanner *bufio.Scanner = bufio.NewScanner(file)
    let (sender, receiver) = std::sync::mpsc::channel::<Board>();
    let mut join_set = tokio::task::JoinSet::<()>::new();
	for line in lines {
    //for scanner.Scan() {

        join_set.spawn(async move {
            let mut board = [[0; 9]; 9];
            let chars: Vec<char> = line.chars().collect();
            read_line(chars, &mut board);
            //print_board(&board);
            solve(&mut board, 0, 0);
            //print_board(&board);
            assert(verify(&board))
        });
	}
    
    join_set.join_all().await;

}

pub async fn main() {
    let input = std::env::args().collect::<Vec<String>>()[2].clone();
    println!("input file: {}", &input);
    let t0 = std::time::Instant::now();
	process(input).await;
    let t1 = std::time::Instant::now();
    println!("Time consumed: {}ms", (t1-t0).as_millis());
}
