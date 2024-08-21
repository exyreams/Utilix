mod app;
mod tools;
mod ui;

use crate::app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tui_textarea::TextArea;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Tool TextAreas
    let mut base64_converter_textarea = TextArea::default();
    let mut date_converter_textarea = TextArea::default();
    let mut hash_generator_textarea = TextArea::default();
    let mut number_base_converter_textarea = TextArea::default();
    let mut qr_code_generator_textarea = TextArea::default();

    let app = App::new();
    let res = ui::run_app(
        &mut terminal,
        app,
        &mut base64_converter_textarea,
        &mut date_converter_textarea,
        &mut hash_generator_textarea,
        &mut number_base_converter_textarea,
        &mut qr_code_generator_textarea,
    );

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
