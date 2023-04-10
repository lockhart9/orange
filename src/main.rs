use std::io::{stdin, stdout, Write};

use termion::cursor::Goto;

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

    writeln!(
        stdout,
        "{}{}",
        termion::color::Fg(termion::color::Red),
        TITLE_TEXT
    )?;

    write!(stdout, "{}", Goto(width / 2, height / 2 + 2))?;

    stdout.flush()?;

    let mut count = 0;
    for event in stdin.events() {
        let event = event?;
        match event {
            termion::event::Event::Key(key) => match key {
                termion::event::Key::Char(_) => {
                    let next_char = match count {
                        0 => 'Y',
                        1 => 'E',
                        2 => 'S',
                        3 => '!',
                        _ => break,
                    };
                    count += 1;
                    write!(stdout, "{}", next_char)?;
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
