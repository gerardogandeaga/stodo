#[allow(dead_code)]

pub mod clap_args;
pub mod stodo_args;
pub mod term;

use {clap_args::clap_args, stodo_args::CliConfig};

pub fn run_config() -> CliConfig {
    let clap_args = clap_args();
    CliConfig::from(&clap_args)
}

use std::{io::{Write}};

use crossterm::{
    queue, 
    cursor,
    terminal,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, 
    Result,
    style::{self, Stylize},
    event::{self, Event, KeyCode, KeyEvent}
};

use self::term::TermData;

#[allow(dead_code)]
pub fn run<W>(writer: &mut W, writable: String) -> Result<()> 
    where
        W: Write, 
{
    let mut term_data = term::TermData::new(writable);

    execute!(writer, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    queue!(
        writer, 
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        // cursor::Show,
        cursor::Hide,
        cursor::EnableBlinking,
        cursor::MoveTo(0,0)
    )?;

    let (_, mut terminal_height) = terminal::size().unwrap();
    render(writer, &term_data)?;

    writer.flush()?;

    'program: loop {

        loop {
            let (_, cursor_y) = cursor::position().unwrap();

            match event::read().unwrap() {
                Event::Resize(_, h) => {
                    terminal_height = h;
                    render(writer, &term_data)?;
                },
                Event::Key(KeyEvent {code: KeyCode::Up, .. }) => {
                    if term_data.point_to_next() {
                        if cursor_y > 0 {
                            queue!(writer, cursor::MoveUp(1))?;
                        }
                        else {
                            term_data.shift_top_up();
                            queue!(writer, cursor::MoveTo(0, 0))?;
                        }
                        

                        render(writer, &term_data)?;
                    }
                },
                Event::Key(KeyEvent {code: KeyCode::Down, .. }) => {
                    if term_data.point_to_prev() {
                        if cursor_y < terminal_height - 1 {
                            queue!(writer, cursor::MoveDown(1))?;
                        }
                        else {
                            term_data.shift_top_down();
                            queue!(writer, cursor::MoveTo(0, terminal_height - 1))?;
                        }
                        render(writer, &term_data)?;
                    }
                },
                Event::Key(KeyEvent {code: KeyCode::Char('q'), .. }) => {
                    break 'program;
                },
                _ => {}
            }

            writer.flush()?;
        }
    }

    execute!(
        writer, style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

fn render<W: Write>(w: &mut W, term_data: &TermData) -> Result<()> {
    let (_, height) = terminal::size().unwrap();
    let (_, y) = cursor::position().unwrap();

    queue!(w, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0))?;

    term_data.buffer_window(height as usize)
        .enumerate()
        .for_each(|(i, line)| {
            if i == y as usize {
                let s = format!(">{}", line);
                queue!(
                    w, 
                    // SetForegroundColor(Color::Yellow), 
                    style::Print(s.as_str().bold()),
                    cursor::MoveToNextLine(1),
                    ResetColor
                ).ok();
            }
            else {
                let s = format!(" {}", line);
                queue!(
                    w, 
                    style::Print(s.as_str()), 
                    cursor::MoveToNextLine(1)
                ).ok();
            }
        });

    queue!(w, cursor::MoveTo(0, y))?;

    w.flush()
}
