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
    // Enable raw mode for crossterm, allowing for direct character input
    enable_raw_mode()?;
    // Get a mutable reference to standard output for interacting with the terminal
    let mut stdout = io::stdout();
    // Enter alternate screen mode, making sure that we clear the screen when exiting
    execute!(stdout, EnterAlternateScreen)?;
    // Create a new terminal backend using crossterm to handle the terminal interface
    let backend = CrosstermBackend::new(stdout);
    // Create a new ratatui terminal with the initialized backend
    let mut terminal = Terminal::new(backend)?;

    // Initialize text areas for each tool, these will be used to input and display data
    let mut base64_converter_textarea = TextArea::default();
    let mut color_converter_textarea = TextArea::default();
    let mut date_converter_textarea = TextArea::default();
    let mut hash_generator_textarea = TextArea::default();
    let mut number_base_converter_textarea = TextArea::default();
    let mut qr_code_generator_textarea = TextArea::default();

    // Create a new instance of the App, the main application structure.
    let app = App::new(); 

    // Run the UI of the application
    // `run_app` manages the user interaction, drawing the UI, and processing tool actions.
    let res = ui::run_app(
        &mut terminal,
        app, // remove &mut
        &mut base64_converter_textarea,
        &mut color_converter_textarea,
        &mut date_converter_textarea,
        &mut hash_generator_textarea,
        &mut number_base_converter_textarea,
        &mut qr_code_generator_textarea,
    );

    // Disable raw mode & returns to normal state.
    disable_raw_mode()?;
    // Exit the alternate screen mode, cleaning up the terminal.
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    // Show the cursor again for regular terminal interactions.
    terminal.show_cursor()?;

    // If any errors occur during the UI execution, print them to the console.
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}