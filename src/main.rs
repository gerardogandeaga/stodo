mod cli;
mod stodo_tree;
mod display;

use std::io::{self, Write, stdout};

use crossterm::{
    queue, 
    cursor,
    terminal,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, 
    Result,
    style,
    event::{self, Event, KeyCode, KeyEvent}
};

// TODO: experiment with the scrolling function in crossterm. Try to get the gutter working.
// Maybe I can use a string builder that creates the display tree. each line is build from left to right so think about the machanics.
// maybe we can create the tree in a string and the gutter in another and append the string line by line.

fn run<W>(w: &mut W, writable: String) -> Result<()> 
    where
        W: Write, 
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    queue!(
        w, 
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        // SetForegroundColor(Color::Blue),
        // SetBackgroundColor(Color::Red),
        // terminal::SetSize(100, 50),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;

    for line in writable.split("\n") {
        queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
    }

    queue!(
        w, 
        ResetColor,
        cursor::Show,
        cursor::EnableBlinking,
    )?;

    w.flush()?;

    loop {

        // custom write
        match read_char()? {
            'w' => execute!(w, terminal::ScrollDown(1))?,
            's' => execute!(w, terminal::ScrollUp(1))?,
            'q' => break,
            _ => {
                // queue!(
                //     w,
                //     style::Print("-> "),
                //     style::Print(x),
                //     cursor::MoveToNextLine(1)
                // )?;
            }
        };

        w.flush()?;
    }

    execute!(
        w, style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            // code: KeyCode::Up,
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

// use termtree::Tree;

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    let stodo_config = cli::run_config();
    let stodo_trees: stodo_tree::StodoForest = stodo_tree::build_stodo_trees(stodo_config.src_paths, stodo_config.recursive);
    // display::print_tree::display_stodo_tree(&stodo_trees);
    let forest_str = display::display_tree::builder::DisplayForestBuilder::compile(&stodo_trees);
    // println!("{}", forest_str);
    // println!("\x1b[93mError\x1b[0m");
    // ----------------------------------------------------------------------------------------------

    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    let mut stdout = io::stdout();
    run(&mut stdout, forest_str)

    // Ok(())
}


