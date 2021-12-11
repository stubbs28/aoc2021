#[path="utils/reader.rs"] mod reader;
use std::collections::HashMap;

pub struct Board {
    tiles: Vec::<i32>,
    visited: Vec::<i32>,
    row: i32,
    col: i32,
}

impl Board {
    pub fn new(row: i32, col: i32) -> Board {
        let size = (row * col).try_into().unwrap();
        Board {
            tiles: Vec::<i32>::with_capacity(size),
            visited: Vec::<i32>::with_capacity(size),
            row: row,
            col: col,
        }
    }

    pub fn add_tile(&mut self, tile: i32, visited: i32) {
        self.tiles.push(tile);
        self.visited.push(visited);
    }

    pub fn score(&self) -> i32 {
        let w = self.winning_turn();
        let mut sum = 0;
        let mut last = 0;
        for i in 0..(self.row * self.col) {
            let pos = i as usize;
            if self.visited[pos] > w {
                sum += self.tiles[pos]
            } else if self.visited[pos] == w {
                last = self.tiles[pos]
            }
        }
        sum * last
    }

    pub fn winning_turn(&self) -> i32 {
        let mut w = 100;
        for i in 0..self.row {
            let r = i * self.col;
            let mut bingor = self.visited[r as usize];
            let mut bingoc = self.visited[i as usize];
            for j in 0..self.col {
                let rround = self.visited[(r + j) as usize];
                if bingor < rround {
                    bingor = rround;
                }
                let cround = self.visited[(i + (j * self.row)) as usize];
                if bingoc < cround {
                    bingoc = cround;
                }
            }
            if w > bingor {
                w = bingor;
            }
            if w > bingoc {
                w = bingoc;
            }
        }
        w
    }
}

pub struct Bingo {
    boards: Vec::<Board>,
}

impl Bingo {
    pub fn new(game: String) -> Bingo {
        println!("{}", game);
        let mut reader = reader::BufReader::open(game).unwrap();
        let mut buffer = String::new();
        let mut rand = HashMap::with_capacity(100);
        let mut boards = Vec::<Board>::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let line = line.unwrap().trim();
            if rand.len() == 0 {
                let l: Vec::<&str> = line.split(",").collect();
                for (i, s) in l.iter().enumerate() {
                    rand.insert(
                        s.parse::<i32>().unwrap(),
                        i as i32,
                    );
                }
                continue;
            }
            if line.len() == 0 {
                boards.push(Board::new(5, 5));
                continue;
            }
            let l: Vec::<&str> = line.split(" ")
                .filter(|s| !s.is_empty())
                .collect();
            for i in l {
                let num = i.parse::<i32>().unwrap();
                let turn = rand[&num];
                if let Some(board) = boards.last_mut() {
                    board.add_tile(num, turn);
                }
            }
        }
        Bingo {
            boards: boards,
        }
    }

    pub fn winning_score(&self) -> i32 {
        let mut b = &self.boards[0];
        let mut t = b.winning_turn();
        for i in &self.boards {
            let wt = i.winning_turn();
            if wt < t {
                b = i;
                t = wt;
            }
        }
        b.score()
    }
}
