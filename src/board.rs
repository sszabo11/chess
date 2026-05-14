use std::{
    io::{Read, Write, stdin, stdout},
    os::unix::process,
    process::Stdio,
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, MouseEventKind, poll, read,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

use crate::{
    event::GameEvent,
    peice::{Color, PieceData, PieceKind, print_piece},
    player::Player,
};

pub struct Board {
    pub white: Player,
    pub black: Player,
    pub rows: usize,
    pub cols: usize,
    pub turn: Color,
    pub pxs_per_square: u32,
    pub focused_square: (u32, u32),
    pub term_board_row: u32,
}

impl Board {
    pub fn new(rows: usize, cols: usize, pxs_per_square: u32) -> Self {
        Self {
            rows,
            cols,
            pxs_per_square,
            white: Player::default(),
            black: Player::default(),
            turn: Color::White,
            focused_square: (0, 0),
            term_board_row: 0,
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

    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!();
        //println!("\x1b[38;5;137m█\x1b[0m");
        //println!("\x1b[48;2;92;64;51m\x1b[30m  ASCII ART  \x1b[0m");

        //let qh = pxs_per_square / 2;
        let qh = (self.pxs_per_square as f32 / 2.0).ceil() as usize;
        for row in 0..self.rows {
            for q in 0..qh {
                for col in 0..self.cols {
                    let bg_options = if (col + row) % 2 == 0 {
                        |c: &str| {
                            (
                                format!("\x1b[48;2;210;180;140m{}\x1b[0m", c),
                                format!("\x1b[48;2;235;215;185m{}\x1b[0m", c),
                            )
                        }
                    } else {
                        |c: &str| {
                            (
                                format!("\x1b[48;2;92;64;51m{}\x1b[0m", c),
                                //format!("\x1b[48;2;92;64;51m{}\x1b[0m", c),
                                format!("\x1b[48;2;160;130;105m{}\x1b[0m", c),
                            )
                        }
                    };

                    let bg = |c: &str| {
                        if self.focused_square.0 == col as u32
                            && self.focused_square.1 == row as u32
                        {
                            bg_options(c).1
                        } else {
                            bg_options(c).0
                        }
                    };

                    let mut place = 0;

                    let half = (qh as f32 / 2.0).ceil() as usize;

                    for piece in self.white.pieces.iter() {
                        let s = print_piece(&piece.kind, Color::Black);
                        if q == half && piece.row == row as u32 && piece.col == col as u32 {
                            let add = if self.pxs_per_square >= 7 { 1 } else { 0 };
                            place += qh;

                            print!("{}", bg(" ").repeat(half + add));
                            print!(
                                "\x1b[38;2;{};{};{};48;2;{};{};{}m{}\x1b[0m",
                                245,
                                235,
                                220,
                                140,
                                90,
                                60,
                                bg(&s)
                            );
                        }
                    }
                    for piece in self.black.pieces.iter() {
                        let s = print_piece(&piece.kind, Color::Black);
                        if q == half && piece.row == row as u32 && piece.col == col as u32 {
                            let add = if self.pxs_per_square >= 7 { 1 } else { 0 };
                            place += qh;

                            print!("{}", bg(" ").repeat(half + add));
                            print!("\x1b[30m{}\x1b[0m", bg(&s));
                        }
                    }

                    print!("{}", bg(" ").repeat(self.pxs_per_square as usize - place));
                }
                println!();
            }
        }

        println!()
    }

    pub fn poll_input(&self) -> Result<GameEvent> {
        enable_raw_mode()?;
        execute!(
            stdout(),
            EnableMouseCapture,
            EnableFocusChange,
            EnableBracketedPaste
        )?;
        loop {
            match read().expect("Failed to read event") {
                Event::FocusGained => {}
                Event::Mouse(event) => match event.kind {
                    MouseEventKind::Down(btn) => {
                        let x = (event.column as f32 / self.pxs_per_square as f32).floor() as u32;
                        let y = (event.row as u32 - self.term_board_row) as u32
                            / (self.pxs_per_square / 2);

                        disable_raw_mode().unwrap();

                        return Ok(GameEvent::SquareHighlight(x, y));
                    }

                    _ => {}
                },
                Event::Key(key) => match key.code {
                    KeyCode::Esc => {
                        execute!(
                            std::io::stdout(),
                            DisableFocusChange,
                            DisableBracketedPaste,
                            DisableMouseCapture
                        )
                        .expect("Failed to reset");
                        disable_raw_mode().unwrap();
                        std::process::exit(1);
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(GameEvent::None)
    }

    pub fn tick(&mut self, event: GameEvent) {
        match event {
            GameEvent::SquareHighlight(row, col) => {
                self.focused_square.0 = row;
                self.focused_square.1 = col;
            }
            _ => {}
        }
    }

    pub async fn wait_for_white(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(
            stdout(),
            EnableMouseCapture,
            EnableFocusChange,
            EnableBracketedPaste
        )?;

        tokio::spawn(async move {
            loop {
                match read().expect("Failed to read event") {
                    Event::FocusGained => println!("FocusGained"),
                    Event::Mouse(event) => match event.kind {
                        MouseEventKind::Down(btn) => {
                            println!("Down at: ({}, {})", event.row, event.column);
                        }
                        _ => {}
                    },
                    Event::Key(key) => match key.code {
                        KeyCode::Esc => {
                            execute!(
                                std::io::stdout(),
                                DisableFocusChange,
                                DisableBracketedPaste,
                                DisableMouseCapture
                            )
                            .expect("Failed to reset");
                            disable_raw_mode().unwrap();
                            std::process::exit(1);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        })
        .await?;

        disable_raw_mode()?;
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
