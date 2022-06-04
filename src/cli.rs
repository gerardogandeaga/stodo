#[allow(dead_code)]
pub mod clap_args;
pub mod stodo_args;
pub mod term;

use {clap_args::clap_args, stodo_args::CliConfig};
use super::display::{StodoOutput, LineToken};

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

/// TODO: Change the cursor movement to only between stodo entries

#[allow(dead_code)]
pub fn run<W>(writer: &mut W, stodo_output: StodoOutput) -> Result<()> 
    where
        W: Write, 
{
    execute!(writer, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    queue!(
        writer, 
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::EnableBlinking,
        cursor::MoveTo(0,0)
    )?;

    let (terminal_width, mut terminal_height) = terminal::size().unwrap();

    // init the data for rendering
    let mut term_data = term::TermData::new(stodo_output, (terminal_height, terminal_width));

    render(writer, &term_data)?;

    writer.flush()?;

    'program: loop {

        loop {
            let (_, cursor_y) = cursor::position().unwrap();

            match event::read().unwrap() {
                Event::Resize(w, h) => {
                    terminal_height = h;
                    term_data.resize_window((h, w));
                    render(writer, &term_data)?;
                },
                Event::Key(KeyEvent {code: KeyCode::Up, .. }) => {
                    // TODO: Handle the case when shifting to a stodo that is out of range from the current screen
                    let shift: usize = term_data.point_to_prev();
                    
                    if shift > 0 {
                        queue!(writer, cursor::MoveTo(0, term_data.cursor_window_position()))?;
                        render(writer, &term_data)?;
                    }
                },
                Event::Key(KeyEvent {code: KeyCode::Down, .. }) => {
                    let shift: usize = term_data.point_to_next();

                    if shift > 0 {
                        queue!(writer, cursor::MoveTo(0, term_data.cursor_window_position()))?;
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
    let (width, height) = terminal::size().unwrap();
    let (_, y) = cursor::position().unwrap();

    queue!(w, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0))?;

    term_data.buffer_window()
        .iter()
        .enumerate()
        .for_each(|(i, (t, line))| {
            // render a division line
            if matches!(t, LineToken::Div) {
                // ─
                let s = format!(" {}", line);
                let ss = s.trim_end();
                let g_width = ss.len();
                queue!(
                    w, 
                    style::Print(ss), 
                    style::Print(format!("{:─>width$}", "", width = width as usize - g_width)), 
                    cursor::MoveToNextLine(1)
                ).ok();
            }
            else
            // render the text line
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
