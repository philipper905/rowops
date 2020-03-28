use termion::{
    event::{Event, Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

use std::io::{self, Write};

mod matrix;
mod opt;

pub const DEFAULT_NUM_ROWS: u8 = 3;
pub const DEFAULT_NUM_COLS: u8 = 4;

fn main() {
    let opt = opt::Opt::create();
    let matrix = matrix::MatrixBuilder::default()
        .columns(opt.cols)
        .rows(opt.rows)
        .build();

    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}q to exit. Click, click, click!",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{} x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
