use std::io::{stdin, stdout, Write};

use termion::cursor::{Goto, Left, Right};

use termion::input::TermRead;
use termion::raw::IntoRawMode;

const TITLE_TEXT: &str = "Would you like an orange?";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = termion::terminal_size()?;

    let mut stdout = stdout().into_raw_mode()?;
    let stdin = stdin().lock();

    write!(stdout, "{}{}", termion::clear::All, Goto(1, 1))?;

    stdout.write_all(format!("Window size = ({}, {})\n", width, height).as_bytes())?;

    write!(
        stdout,
        "{}",
        Goto(width / 2 - (TITLE_TEXT.len() as u16) / 2, height / 2)
    )?;

    stdout.write_all(format!("{}\n", TITLE_TEXT).as_bytes())?;
    stdout.flush()?;

    for event in stdin.events() {
        let event = event?;
        match event {
            termion::event::Event::Key(key) => match key {
                termion::event::Key::Left => {
                    write!(stdout, "{}", Left(1))?;
                    stdout.flush()?;
                }
                termion::event::Key::Right => {
                    write!(stdout, "{}", Right(1))?;
                    stdout.flush()?;
                }
                termion::event::Key::Char(c) if c == 'q' => break,
                termion::event::Key::Char(c) => {
                    write!(stdout, "{}", c)?;
                    stdout.flush()?;
                }
                termion::event::Key::Esc => break,
                _ => {}
            },
            termion::event::Event::Mouse(_) => todo!(),
            termion::event::Event::Unsupported(_) => todo!(),
        }
    }

    write!(stdout, "{}", Goto(width, height))?;

    Ok(())
}
