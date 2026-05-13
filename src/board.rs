use std::{io::stdout, process::Stdio};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin},
    process::Command,
};

use crate::{
    peice::{Color, PieceData, PieceKind, print_piece},
    player::Player,
};

pub struct Board {
    pub white: Player,
    pub black: Player,
    pub rows: usize,
    pub cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            white: Player::default(),
            black: Player::default(),
        }
    }

    //pub fn init_white<const N: usize>(mut self, pos: Vec<[fn(u32, u32) -> PieceKind; N]>) -> Self {
    //    for row in 0..pos.len() {
    //        for (col, &piece) in pos[row].iter().enumerate() {
    //            self.white.pieces.push(piece(row as u32, col as u32));
    //        }
    //    }

    //    self
    //}
    //let init_pos_white: Vec<[fn(u32, u32) -> Piece; 8]>
    pub fn init_white<const N: usize>(mut self, pos: Vec<[PieceKind; N]>) -> Self {
        for row in 0..pos.len() {
            for (col, &piece) in pos[row].iter().enumerate() {
                let data = PieceData {
                    row: row as u32 + self.rows as u32 - 2,
                    col: col as u32,
                    kind: piece,
                };
                self.white.pieces.push(data);
            }
        }

        self
    }

    pub fn init_black<const N: usize>(mut self, pos: Vec<[PieceKind; N]>) -> Self {
        for row in 0..pos.len() {
            for (col, &piece) in pos[row].iter().enumerate() {
                let data = PieceData {
                    row: row as u32,
                    col: col as u32,
                    kind: piece,
                };
                self.black.pieces.push(data);
            }
        }
        self
    }

    pub fn draw(&self, pxs_per_square: usize) {
        print!("\x1B[2J\x1B[1;1H");
        println!();
        //println!("\x1b[38;5;137m█\x1b[0m");
        //println!("\x1b[48;2;92;64;51m\x1b[30m  ASCII ART  \x1b[0m");

        //let qh = pxs_per_square / 2;
        let qh = (pxs_per_square as f32 / 2.0).ceil() as usize;
        for row in 0..self.rows {
            for q in 0..qh {
                for col in 0..self.cols {
                    let bg = if (col + row) % 2 == 0 {
                        |c: &str| format!("\x1b[48;2;210;180;140m{}\x1b[0m", c)
                    } else {
                        |c: &str| format!("\x1b[48;2;92;64;51m{}\x1b[0m", c)
                    };

                    let mut place = 0;

                    let half = (qh as f32 / 2.0).ceil() as usize;

                    for piece in self.white.pieces.iter() {
                        let s = print_piece(&piece.kind, Color::White);
                        if q == half && piece.row == row as u32 && piece.col == col as u32 {
                            let add = if pxs_per_square >= 7 { 1 } else { 0 };
                            place += qh;

                            print!("{}", bg(" ").repeat(half + add));
                            print!("\x1b[30m{}\x1b[0m", bg(&s));
                        }
                    }
                    for piece in self.black.pieces.iter() {
                        let s = print_piece(&piece.kind, Color::Black);
                        if q == half && piece.row == row as u32 && piece.col == col as u32 {
                            let add = if pxs_per_square >= 7 { 1 } else { 0 };
                            place += qh;

                            print!("{}", bg(" ").repeat(half + add));
                            print!("\x1b[30m{}\x1b[0m", bg(&s));
                        }
                    }

                    print!("{}", bg(" ").repeat(pxs_per_square - place));
                }
                println!();
            }
        }

        println!()
    }

    pub async fn wait_for_white(&mut self) -> Result<()> {
        //tokio::spawn(async move {
        //    let mut stdout = stdout();

        //    let res = stdout
        //        .write(b"\x1b[6n")
        //        .await
        //        .expect("Failed to write to stdout");
        //    print!("{}", res);
        //});
        let mut stdout = stdout().into_raw().unwrap();
        let mut stdin = stdin();

        let res = stdout
            .write(b"\x1b[6n")
            .await
            .expect("Failed to write to stdout");
        print!("'{}' ", res);

        let mut buf = vec![];
        let res = stdin
            .read(&mut buf)
            .await
            .expect("Failed to write to stdout");

        print!("r: {}, Buf: {:?}", res, buf);
        Ok(())
    }
    pub async fn wait_for_black(&mut self) -> Result<()> {
        let mut child = Command::new("bash")
            .arg("-c")
            .arg("echo 'Status: Start'; sleep 1; echo 'Status: End'")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn");

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            println!("length = {}", line)
        }

        child.wait().await.expect("Failed to wait");
        Ok(())
    }
}
