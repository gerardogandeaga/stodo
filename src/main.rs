mod cli;
mod stodo_tree;
mod display;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

extern crate petgraph;

// use petgraph::dot::Dot;
const MENU: &str = r#"Crossterm interactive test
Controls:
 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step
Available tests:
1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)
4. input
Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

// TODO: experiment with the scrolling function in crossterm. Try to get the gutter working.
// Maybe I can use a string builder that creates the display tree. each line is build from left to right so think about the machanics.
// maybe we can create the tree in a string and the gutter in another and append the string line by line.

/*
 *       |
 *   5[x]|
 *  12[o]|
 * 100[x]|
 */

fn run<W>(w: &mut W) -> Result<()> 
    where
        W: Write, {

    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    queue!(
        w, 
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        // terminal::SetSize(100, 50),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;

    for line in MENU.split('\n') {
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

        // let y = cursor::position().unwrap();
        // println!("({}, {})", y.0, y.1);

        // custom write

        match read_char()? {
            'w' => execute!(w, terminal::ScrollDown(1))?,
            's' => execute!(w, terminal::ScrollUp(1))?,
            'q' => break,
            x => {
                queue!(
                    w,
                    style::Print("-> "),
                    style::Print(x),
                    cursor::MoveToNextLine(1)
                )?;
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

fn main() -> Result<()> {
    let stodo_config = cli::run_config();

    let stodo_trees = stodo_tree::build_stodo_trees(stodo_config.src_paths, stodo_config.recursive);

    display::print_tree::display_stodo_tree(&stodo_trees);

    // display::builder::test();

    // stodo_trees.into_iter()
    //     .for_each(|tree| println!("{}", Dot::new(&tree)));

    // execute!(
    //     stdout(),
    //     EnterAlternateScreen
    // )?;


    // thread::sleep(Duration::from_millis(2 * 1000));

    

    // loop {
    //     execute!(
    //         stdout(),
    //         SetForegroundColor(Color::Blue),
    //         SetBackgroundColor(Color::Red),
    //         Print("Styled text here.\n"),
    //         ResetColor
    //     )?;
    // }

    

    // execute!(stdout(), LeaveAlternateScreen)?;
    // let mut stdout = io::stdout();
    // run(&mut stdout)

    Ok(())
}


