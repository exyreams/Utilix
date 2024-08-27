use crate::app::{App, Tool};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use std::io;
use tui_textarea::{CursorMove, Input, Key, TextArea};

/// Runs the main application loop, handling user input and rendering the User Interface.
///
/// This function is responsible for managing the interaction between the user,
/// the terminal, and the various tools available in the application.
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    base64_converter_textarea: &mut TextArea,
    color_converter_textarea: &mut TextArea,
    date_converter_textarea: &mut TextArea,
    hash_generator_textarea: &mut TextArea,
    number_base_converter_textarea: &mut TextArea,
    qr_code_generator_textarea: &mut TextArea,
) -> io::Result<()> {
    // Start an infinite loop, continuously handling user input and redrawing the User Interface.
    loop {
        // Draw application UI.
        terminal.draw(|f| {
            ui(
                f,                              // A mutable reference to the terminal's frame.
                &mut app,                       // A mutable reference to the application state.
                base64_converter_textarea,      // Text area for base64 conversion.
                color_converter_textarea,       // Text area for color code conversion.
                date_converter_textarea,        // Text area for date conversion.
                hash_generator_textarea,        // Text area for hash generation.
                number_base_converter_textarea, // Text area for number base conversion.
                qr_code_generator_textarea,     // Text area for QR code generation.
            )
        })?;

        // Read the next user input event (keyboard, mouse).
        match crossterm::event::read()? {
            // Handle keyboard inputs
            crossterm::event::Event::Key(key) => {
                // Create an input object representing the keyboard key press.
                let input = match key.code {
                    KeyCode::Char(c) => Input {
                        key: Key::Char(c),    // Convert the character code into a Key::Char variant.
                        ..Default::default() // Initialize remaining input fields with their defaults.
                    },

                    // Handle Esc key to quit the application.
                    KeyCode::Esc => Input {
                        key: Key::Esc,
                        ..Default::default()
                    },

                    // Switch between different tools using the Tab key.
                    KeyCode::Tab => {
                        app.current_tool = match app.current_tool {
                            Tool::Base64Encoder => Tool::ColorConverter,
                            Tool::ColorConverter => Tool::DateConverter,
                            Tool::DateConverter => Tool::HashGenerator,
                            Tool::HashGenerator => Tool::NumberBaseConverter,
                            Tool::NumberBaseConverter => Tool::PasswordGenerator,
                            Tool::PasswordGenerator => Tool::QRCodeGenerator,
                            Tool::QRCodeGenerator => Tool::UuidGenerator,
                            Tool::UuidGenerator => Tool::Base64Encoder,
                        };
                        // Skip processing of other input events this time as switching tools is a dedicated action.
                        continue;
                    }

                    // Handle Left, Right, Up, and Down keys to move cursor.
                    KeyCode::Left => {
                        // Handle Ctrl + Left Arrow to move cursor word back.
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                            color_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::WordBack);
                            number_base_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::WordBack);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                        // Handle Shift + Left Arrow to select the entire line (to the left).
                        } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);

                            color_converter_textarea.move_cursor(CursorMove::Head);
                            color_converter_textarea.start_selection();
                            color_converter_textarea.move_cursor(CursorMove::End);

                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);

                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);

                            number_base_converter_textarea.move_cursor(CursorMove::Head);
                            number_base_converter_textarea.start_selection();
                            number_base_converter_textarea.move_cursor(CursorMove::End);

                            qr_code_generator_textarea.move_cursor(CursorMove::Head);
                            qr_code_generator_textarea.start_selection();
                            qr_code_generator_textarea.move_cursor(CursorMove::End);
                        // Handle normal Left Arrow to move cursor back one character.
                        } else {
                            base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            base64_converter_textarea.cancel_selection();

                            color_converter_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            color_converter_textarea.cancel_selection();

                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            date_converter_textarea.cancel_selection();

                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            hash_generator_textarea.cancel_selection();

                            number_base_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::Back);
                            number_base_converter_textarea.cancel_selection();

                            qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            qr_code_generator_textarea.cancel_selection();
                        }
                        // Continue the loop as cursor movement is a continuous action.
                        continue;
                    }

                    // Handle Right, Up, and Down arrow keys similar to Left Arrow,
                    //  but for corresponding directions.
                    KeyCode::Right => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            color_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            date_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            hash_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            number_base_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                        } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);

                            color_converter_textarea.move_cursor(CursorMove::Head);
                            color_converter_textarea.start_selection();
                            color_converter_textarea.move_cursor(CursorMove::End);

                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);

                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);

                            number_base_converter_textarea.move_cursor(CursorMove::Head);
                            number_base_converter_textarea.start_selection();
                            number_base_converter_textarea.move_cursor(CursorMove::End);

                            qr_code_generator_textarea.move_cursor(CursorMove::Head);
                            qr_code_generator_textarea.start_selection();
                            qr_code_generator_textarea.move_cursor(CursorMove::End);
                        } else {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::Forward);
                            base64_converter_textarea.cancel_selection();

                            color_converter_textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            color_converter_textarea.cancel_selection();

                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            date_converter_textarea.cancel_selection();

                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            hash_generator_textarea.cancel_selection();

                            number_base_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::Forward);
                            number_base_converter_textarea.cancel_selection();

                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::Forward);
                            qr_code_generator_textarea.cancel_selection();
                        }
                        continue;
                    }
                    // Handle Right, Up, and Down arrow keys similar to Left Arrow,
                    //  but for corresponding directions.
                    KeyCode::Up => {
                        base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        color_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        date_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        number_base_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Up);

                        continue;
                    }
                    // Handle Right, Up, and Down arrow keys similar to Left Arrow,
                    //  but for corresponding directions.
                    KeyCode::Down => {
                        base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        color_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        date_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        number_base_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Down);

                        continue;
                    }

                    // Handle Enter key to insert a newline character.
                    KeyCode::Enter => {
                        base64_converter_textarea.insert_newline();
                        color_converter_textarea.insert_newline();
                        date_converter_textarea.insert_newline();
                        hash_generator_textarea.insert_newline();
                        number_base_converter_textarea.insert_newline();
                        qr_code_generator_textarea.insert_newline();
                        continue;
                    }

                    // Handle Backspace key to delete the previous character.
                    KeyCode::Backspace => {
                        base64_converter_textarea.delete_char();
                        color_converter_textarea.delete_char();
                        date_converter_textarea.delete_char();
                        hash_generator_textarea.delete_char();
                        number_base_converter_textarea.delete_char();
                        qr_code_generator_textarea.delete_char();
                        continue;
                    }

                    // Handle Delete key to delete the next character.
                    KeyCode::Delete => {
                        base64_converter_textarea.delete_next_char();
                        color_converter_textarea.delete_next_char();
                        date_converter_textarea.delete_next_char();
                        hash_generator_textarea.delete_next_char();
                        number_base_converter_textarea.delete_next_char();
                        qr_code_generator_textarea.insert_newline();
                        continue;
                    }
                    _ => continue, // Continue loop if input is not recognized.
                };

                // Process keyboard input based on the current selected tool.
                match input.key {
                    Key::Esc => return Ok(()), // Exit the application if Esc key is pressed.

                    Key::Char(c) => match app.current_tool {
                        //  Handles functionality for Base64 Encoder
                        Tool::Base64Encoder => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                base64_converter_textarea.insert_char(c);
                            }

                            //  Automatically insert newlines for better readability in base64 output (every 84 characters)
                            if base64_converter_textarea.lines().join("\n").len() % 84 == 0 {
                                base64_converter_textarea.insert_newline();
                            }

                            // Clear export message when new input is received
                            app.base64_encoder.tools_export_message = None;

                            // Shortcut Key (Alt + e) to Encode the input.
                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'e' {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.encode();
                            // Shortcut Key (Alt + d) to Decode the input.
                            } else if key.modifiers.contains(KeyModifiers::ALT) && c == 'd' {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.decode();
                            // Shortcut Key (Alt + x) to Export encoded/decoded output.
                            } else if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                match app.base64_encoder.write_to_file() {
                                    Ok(_) => {
                                        app.base64_encoder.tools_export_message = Some(
                                            "Successfully exported to export/base64.txt"
                                                .to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        app.base64_encoder.tools_export_message =
                                            Some(format!("Failed to export: {}", err));
                                    }
                                }
                            } else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            // Starts encoding/decoding automatically when Text area has input.
                            {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.encode();
                                app.base64_encoder.decode();
                            }
                        }

                        Tool::ColorConverter => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                color_converter_textarea.insert_char(c);
                            }

                            // Shortcut Key (Alt + x) to export the input.
                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                match app.color_converter.export_color_codes() {
                                    Ok(_) => {
                                        app.color_converter.tools_export_message = Some(
                                            "Successfully exported to export/color_codes.txt"
                                                .to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        app.color_converter.tools_export_message =
                                            Some(format!("Failed to export: {}", err));
                                    }
                                }
                            } else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            // Starts encoding/decoding automatically when Text area has input.
                            {
                                app.color_converter.input =
                                    color_converter_textarea.lines().join("\n");
                                app.color_converter.convert_all();
                            }
                        }

                        //  Handles functionality for Date Converter
                        Tool::DateConverter => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                date_converter_textarea.insert_char(c);
                            }

                            // Automatically insert newlines for better readability in date output (every 50 characters).
                            if date_converter_textarea.lines().join("\n").len() % 50 == 0 {
                                date_converter_textarea.insert_newline();
                            }
                            // Update the date converter's input with the text from the TextArea.
                            app.date_converter.input = date_converter_textarea.lines().join("\n");
                            // Automatically Convert the input date to all supported formats.
                            app.date_converter.convert_all();
                        }

                        //  Handles functionality for Hash Generator
                        Tool::HashGenerator => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                hash_generator_textarea.insert_char(c);
                            }

                            //  Automatically insert newlines for better readability in hash output (every 62 characters).
                            if hash_generator_textarea.lines().join("\n").len() % 62 == 0 {
                                hash_generator_textarea.insert_newline();
                            }

                            // Shortcut Key (Alt + x) to Export generated hashes.
                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                match app.hash_generator.write_to_file() {
                                    Ok(_) => {
                                        app.hash_generator.tools_export_message = Some(
                                            "Successfully exported to export/hash.txt".to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        app.hash_generator.tools_export_message =
                                            Some(format!("Failed to export: {}", err));
                                    }
                                }
                            // If ALT is not pressed, update the input string and calculate hashes.
                            } else if !key.modifiers.contains(KeyModifiers::ALT) {
                                let new_input = hash_generator_textarea.lines().join("\n");
                                app.hash_generator.update_input(&new_input);
                            }
                        }

                        //  Handles functionality for Number base converter.
                        Tool::NumberBaseConverter => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                number_base_converter_textarea.insert_char(c);
                            }
                            //  Automatically insert newlines for better readability in number conversion output (every 50 characters).
                            if number_base_converter_textarea.lines().join("\n").len() % 50 == 0 {
                                number_base_converter_textarea.insert_newline();
                            }

                            // Shortcut Key (Alt + x) to Export number base conversions.
                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                match app.number_base_converter.write_to_file() {
                                    Ok(_) => {
                                        app.number_base_converter.tools_export_message = Some(
                                            "Successfully exported to export/number_conversion.txt"
                                                .to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        app.number_base_converter.tools_export_message =
                                            Some(format!("Failed to export: {}", err));
                                    }
                                }
                            }
                            // Starts number conversion automatically, when input/Text area have characters.
                            else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            {
                                app.number_base_converter.input =
                                    number_base_converter_textarea.lines().join("\n");
                                app.number_base_converter.convert();
                            }
                        }

                        //  Handles functionality for password generator.
                        Tool::PasswordGenerator => match c {
                            // Shortcut Keys, Character mapped to each respective functionality i.e
                            // generate, increase/decrease length, toggle uppercase/lowercase/symbols
                            // toggle similar characters/duplicate characters, clearing etc.
                            'g' => {
                                let _ = app.password_generator.generate_password();
                            }
                            'i' => {
                                app.password_generator.increase_length();
                            }
                            'd' => {
                                app.password_generator.decrease_length();
                            }
                            'u' => {
                                app.password_generator.toggle_uppercase();
                            }
                            'l' => {
                                app.password_generator.toggle_lowercase();
                            }
                            'n' => {
                                app.password_generator.toggle_numbers();
                            }
                            's' => {
                                app.password_generator.toggle_symbols();
                            }
                            'z' => {
                                app.password_generator.toggle_similar_characters();
                            }
                            'q' => {
                                app.password_generator.toggle_duplicate_characters();
                            }
                            'c' => {
                                app.password_generator.clear_password();
                            }
                            'v' => {
                                app.password_generator.toggle_sequential_characters();
                            }
                            'm' => {
                                let _ = app.password_generator.generate_multiple_passwords();
                            }
                            'k' => {
                                app.password_generator.increase_quantity();
                            }
                            'j' => {
                                app.password_generator.decrease_quantity();
                            }
                            'x' => match app.password_generator.write_to_file() {
                                Ok(_) => {
                                    app.password_generator.tools_export_message = Some(
                                        "Successfully exported to export/password.txt".to_string(),
                                    );
                                }
                                Err(err) => {
                                    app.password_generator.tools_export_message =
                                        Some(format!("Failed to export: {}", err));
                                }
                            },
                            _ => {}
                        },

                        //  Handles functionality for QR Code generator.
                        Tool::QRCodeGenerator => {
                            // Only insert characters if ALT and CTRL are not pressed, this
                            // prevents inserting of characters in the text area that shortcuts
                            // for eg. if Alt + e is pressed it will not capture character "e"
                            // and add it on the Text area/Input Field.
                            if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                qr_code_generator_textarea.insert_char(c);
                            }

                            // Automatically insert newlines for better readability in QR code input (every 68 characters).
                            if qr_code_generator_textarea.lines().join("\n").len() % 68 == 0 {
                                qr_code_generator_textarea.insert_newline();
                            }

                            // Shortcut Key (Alt + q) to generate qrcode, it autogenerate doesn't work.
                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'q' {
                                app.qr_code_generator.input =
                                    qr_code_generator_textarea.lines().join("\n");
                                app.qr_code_generator.generate_qr_code();
                            // Shortcut Key (Alt + x) to Export generated qrcode.
                            } else if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                match app.qr_code_generator.export_qr_code() {
                                    Ok(_) => {
                                        app.qr_code_generator.tools_export_message = Some(
                                            "Successfully exported to export/{startinginput}.png"
                                                .to_string(),
                                        );
                                    }
                                    Err(err) => {
                                        app.qr_code_generator.tools_export_message =
                                            Some(format!("Failed to export: {}", err));
                                    }
                                }
                                // Starts number conversion automatically, when input/Text area have characters.
                            } else if !key.modifiers.contains(KeyModifiers::ALT) {
                                app.qr_code_generator.input =
                                    qr_code_generator_textarea.lines().join("\n");
                                app.qr_code_generator.generate_qr_code();
                            }
                        }

                        //  Handles functionality for UUID generator.
                        Tool::UuidGenerator => match c {
                            // Shortcut Keys, Character mapped to each respective functionality i.e
                            // v4 generations, clear, increase/decrease length
                            's' => {
                                app.uuid_generator.generate_v4_uuid();
                            }
                            'm' => {
                                app.uuid_generator.generate_multiple_v4_uuids();
                            }
                            'w' => {
                                app.uuid_generator.generate_v7_uuid();
                            }
                            'e' => {
                                app.uuid_generator.generate_multiple_v7_uuids();
                            }
                            'c' => {
                                app.uuid_generator.clear();
                            }
                            'i' => {
                                app.uuid_generator.increase_length();
                            }
                            'd' => {
                                app.uuid_generator.decrease_length();
                            }
                            'x' => match app.uuid_generator.write_to_file() {
                                Ok(_) => {
                                    app.uuid_generator.tools_export_message = Some(
                                        "Successfully exported to export/uuid.txt".to_string(),
                                    );
                                }
                                Err(err) => {
                                    app.uuid_generator.tools_export_message =
                                        Some(format!("Failed to export: {}", err));
                                }
                            },
                            _ => {}
                        },
                    },
                    //  If no input matches the recognized above inputs for a given tool -  do nothing.
                    _ => {}
                }
            }
            _ => continue, // Ignore events other than keyboard input.
        }
    }
}

// Handles the user interface based on the selected tool and app state.
// Takes the frame, application state, and the input text areas for different tools as arguments.
fn ui(
    f: &mut Frame,
    app: &mut App,
    base64_converter_textarea: &mut TextArea,
    color_converter_textarea: &mut TextArea,
    date_converter_textarea: &mut TextArea,
    hash_generator_textarea: &mut TextArea,
    number_base_converter_textarea: &mut TextArea,
    qr_code_generator_textarea: &mut TextArea,
) {
    // Split the frame into three chunks: title, tabs, and tool content area.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4), // Title area height
            Constraint::Length(3), // Tabs area height
            Constraint::Min(1),    // Tool content area takes the remaining space
        ])
        .split(f.area());

    // Render the title.
    let title = Paragraph::new(
        "█░█ ▀█▀ █ █░░ █ ▀▄▀  Tools Collection\n█▄█ ░█░ █ █▄▄ █ █░█      by @exyreams",
    )
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(title, chunks[0]);

    // Render the tabs for different tools.
    let tabs = Tabs::new(vec![
        Span::raw("Base64 Encoder"),
        Span::raw("Color Code Converter"),
        Span::raw("Date Converter"),
        Span::raw("Hash Generator"),
        Span::raw("Number Base Generator"),
        Span::raw("Password Generator"),
        Span::raw("QR Code Generator"),
        Span::raw("UUID Generator"),
    ])
    .block(
        Block::bordered()
            .title(" Tools ")
            .style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            )
            .border_type(BorderType::Rounded),
    )
    .style(Style::default())
    .highlight_style(Style::default().bg(Color::LightMagenta))
    // Select the currently active tool.
    .select(match app.current_tool {
        Tool::Base64Encoder => 0,
        Tool::ColorConverter => 1,
        Tool::DateConverter => 2,
        Tool::HashGenerator => 3,
        Tool::NumberBaseConverter => 4,
        Tool::PasswordGenerator => 5,
        Tool::QRCodeGenerator => 6,
        Tool::UuidGenerator => 7,
    })
    .divider("|")
    .padding(" ", " ");
    f.render_widget(tabs, chunks[1]);

    // Get the content area for displaying the selected tool's UI.
    let tool_content_area = chunks[2];

    // Render the UI based on the selected tool.
    match app.current_tool {
        Tool::Base64Encoder => base64_encoder(f, app, tool_content_area, base64_converter_textarea),
        Tool::ColorConverter => {
            color_code_converter(f, app, tool_content_area, color_converter_textarea)
        }
        Tool::DateConverter => date_converter(f, app, tool_content_area, date_converter_textarea),
        Tool::HashGenerator => hash_generator(f, app, tool_content_area, hash_generator_textarea),
        Tool::NumberBaseConverter => {
            number_base_converter(f, app, tool_content_area, number_base_converter_textarea)
        }
        Tool::PasswordGenerator => password_generator(f, app, tool_content_area),
        Tool::QRCodeGenerator => {
            qr_code_generator(f, app, tool_content_area, qr_code_generator_textarea)
        }
        Tool::UuidGenerator => uuid_generator(f, app, tool_content_area),
    }
}

// Displays a message within a given area of the frame, indicating either success or an error.
fn tools_export_message(f: &mut Frame, area: Rect, message: &str) {
    // Create a Paragraph with the message, style it based on the message content (green for success, red for error),
    // and center the text.
    let text = Paragraph::new(message)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(
            if message.starts_with("Successfully") {
                Color::Green
            } else {
                Color::Red
            },
        ))
        .alignment(Alignment::Center);

    // Render the message within the provided area.
    f.render_widget(text, area);
}

// Handles the UI for Base64 encoding and decoding.
fn base64_encoder(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    base64_converter_textarea: &mut TextArea,
) {
    // Split the area into three chunks: input/guide area, encoded/decoded area, and a message area.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45), // Input/guide area takes 45% of the height
            Constraint::Percentage(55), // Encoded/decoded area takes 55% of the height
            Constraint::Length(1),      // Message area takes 1 line
        ])
        .split(area);

    // Render the export message if one is available.
    if let Some(message) = &app.base64_encoder.tools_export_message {
        tools_export_message(f, chunks[2], message);
    }

    // Split the input/guide area horizontally.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Style the input area.
    base64_converter_textarea.set_block(
        Block::default()
            .title(" Input ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .padding(Padding::new(1, 1, 0, 0))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded),
    );

    base64_converter_textarea.set_style(Style::default().bold());

    // Render the input text area.
    f.render_widget(&*base64_converter_textarea, input_guide_chunks[0]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("             Quit", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("             Switch Tools", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + e",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Encode", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Decode", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Export Generated Hash", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   export/base64.txt", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Note:",
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::UNDERLINED)
            ),
            Span::styled(
                " Encoding/decoding begins automatically while typing. If it doesn't start, use the shortcut keys.",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::BOLD)
            ),
        ]),
    ];

    // Render the guide text with style and padding.
    let guide = Paragraph::new(guide_text)
        .block(
            Block::default()
                .title(" Base64 Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, input_guide_chunks[1]);

    // Split the encoded/decoded area vertically.
    let encoded_decoded_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Render the encoded value with styling and borders.
    let encoded = Paragraph::new(app.base64_encoder.encoded.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Encoded ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(encoded, encoded_decoded_chunks[0]);

    // Render the decoded value with styling and borders.
    let decoded = Paragraph::new(app.base64_encoder.decoded.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )
        .block(
            Block::default()
                .title(" Decoded ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(decoded, encoded_decoded_chunks[1]);
}

// Handles the UI for color code converter.
fn color_code_converter(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    color_converter_textarea: &mut TextArea,
) {
    // Split the area into two chunks: input/guide area and conversion results area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    // Split the input/guide area into two chunks: input area and guide/status area.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    // Style the input text area.
    color_converter_textarea.set_block(
        Block::default()
            .title(" Enter Color Code ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0)),
    );

    color_converter_textarea.set_style(Style::default().bold());

    // Render the input text area.
    f.render_widget(&*color_converter_textarea, input_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(input_guide_chunks[1]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "        Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "        Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "    Export Colors",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " export/color_codes.txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Examples:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "CMYK:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 0, 26, 99, 1 or 0%, 26%, 99%, 1%",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "HEX:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " #fcba03 or #FCBA03 ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "HSL:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 44, 98, 50 or 44°, 98%, 50% ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "RGB:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 252, 186, 3 ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" Color Converter Help ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Create a status block to display messages or errors.
    let status_text = if let Some(message) = &app.color_converter.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };

    // Render the status block.
    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Split the conversion results area into four chunks for displaying CMYK, HEX, HSL, and RGB.
    let conversion_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    // Render the CMYK value.
    let cmyk = Paragraph::new(app.color_converter.cmyk.clone())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" CMYK Color Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(cmyk, conversion_chunks[0]);

    // Render the HEX value.
    let hex = Paragraph::new(app.color_converter.hex.clone())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" HEX Color Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(hex, conversion_chunks[1]);

    // Render the HSL value.
    let hsl = Paragraph::new(app.color_converter.hsl.clone())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" HSL Color Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(hsl, conversion_chunks[2]);

    // Render the RGB value.
    let rgb = Paragraph::new(app.color_converter.rgb.clone())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" RGB Color Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(rgb, conversion_chunks[3]);
}

// Handles the UI for date converter.
fn date_converter(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    date_converter_textarea: &mut TextArea,
) {
    // Split the area into two chunks: input/guide area and conversion results area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    // Split the input/guide area into two chunks: input area and guide area.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[0]);

    // Style the input text area.
    date_converter_textarea.set_block(
        Block::default()
            .title(" Enter Date ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0)),
    );

    date_converter_textarea.set_style(Style::default().bold());

    // Render the input text area.
    f.render_widget(&*date_converter_textarea, input_guide_chunks[0]);

    // Create a guide text with supported formats, examples, and shortcut keys.
    let guide_text = vec![
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "    Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "    Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Supported Formats:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "YYYY-MM-DD H:M:S",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "YYYY-MM-DDTH:M:S:z",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "YYYY-MM-DD",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "DD/MM/YYYY H:M:S",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "DD/MM/YYYY  ",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Examples:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "RFC 3339:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 2024-05-22T13:00:00Z",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "RFC 2822:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " Tue, 22 May 2022 13:00:00 +0100",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "ISO 8601:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 2024-05-22T13:00:00+01:00 or 20240522T130000+0100",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Unix Timestamp:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 1716382800",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Human Readable:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " Tuesday, March 1, 2022, 1:00:00 PM",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Short Date:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 05/22/2024 or 2024-03-22",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" Date Converter Help ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, input_guide_chunks[1]);

    // Split the conversion results area into two chunks.
    let converstion_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    // Split the first chunk of conversion results area further.
    let converstion_chunks_first_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(converstion_chunks[0]);

    // Split the top portion of the first chunk again.
    let converstion_chunks_second_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(converstion_chunks_first_split[0]);

    // Render the RFC 3339 conversion result.
    let rfc3339_text = vec![Line::from(vec![Span::styled(
        app.date_converter.rfc3339.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let rfc3339 = Paragraph::new(rfc3339_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" RFC 3339 Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(rfc3339, converstion_chunks_second_split[0]);

    // Render the RFC 2822 conversion result.
    let rfc2822_text = vec![Line::from(vec![Span::styled(
        app.date_converter.rfc2822.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let rfc2822 = Paragraph::new(rfc2822_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" RFC 2822 Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(rfc2822, converstion_chunks_second_split[1]);

    // Split the second chunk of the conversion results area.
    let converstion_chunks_third_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(converstion_chunks[1]);

    // Split the first portion of the second chunk.
    let converstion_chunks_third_split_half = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(converstion_chunks_third_split[0]);

    // Render the Time Only conversion result.
    let timeonly_text = vec![Line::from(vec![Span::styled(
        app.date_converter.time_only.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let timeonly = Paragraph::new(timeonly_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Time Only Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(timeonly, converstion_chunks_third_split_half[0]);

    // Render the ISO 8601 conversion result.
    let iso8601_text = vec![Line::from(vec![Span::styled(
        app.date_converter.iso8601.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let iso8601 = Paragraph::new(iso8601_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" ISO 8601 Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(iso8601, converstion_chunks_third_split_half[1]);

    // Render the Unix Timestamp conversion result.
    let unixtimestamp_text = vec![Line::from(vec![Span::styled(
        app.date_converter.unix_timestamp.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let unixtimestamp = Paragraph::new(unixtimestamp_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Unix Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(unixtimestamp, converstion_chunks_third_split[1]);

    // Split the second portion of the first chunk.
    let converstion_chunks_fourth_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(converstion_chunks_first_split[1]);

    // Render the Human Readable conversion result.
    let humanreadable_text = vec![Line::from(vec![Span::styled(
        app.date_converter.human_readable.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let humanreadable = Paragraph::new(humanreadable_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Human Readable Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(humanreadable, converstion_chunks_fourth_split[0]);

    // Render the Short Date conversion result.
    let shordate_text = vec![Line::from(vec![Span::styled(
        app.date_converter.short_date.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let shortdate = Paragraph::new(shordate_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Short Date Conversion ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(shortdate, converstion_chunks_fourth_split[1]);
}

// Handles the UI for hash generator.
fn hash_generator(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    hash_generator_textarea: &mut TextArea,
) {
    // Split the area into two chunks: input/guide area and hash output area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    // Split the input/guide area into two chunks: input area and guide/status area.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Style the input text area.
    hash_generator_textarea.set_block(
        Block::default()
            .title(" Input ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(1, 1, 0, 0)),
    );
    f.render_widget(&*hash_generator_textarea, input_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(input_guide_chunks[1]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "       Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "       Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Export Generated Hash",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " export/hash.txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" Hash Generator Help ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Render the status block for messages and errors.
    let status_text = if let Some(message) = &app.hash_generator.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };
    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Split the hash output area into two chunks for SHA-1/SHA-256 and SHA-384/SHA-512.
    let hash_output_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Split the top chunk into SHA-1 and SHA-256.
    let sha1_sha256_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(hash_output_chunks[0]);

    // Render the SHA-1 hash value.
    let sha1 = Paragraph::new(app.hash_generator.get_sha1())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" SHA-1 Hash ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(sha1, sha1_sha256_chunks[0]);

    // Render the SHA-256 hash value.
    let sha256 = Paragraph::new(app.hash_generator.get_sha256())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" SHA-256 Hash ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(sha256, sha1_sha256_chunks[1]);

    // Split the bottom chunk into SHA-384 and SHA-512.
    let sha384_sha512_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(hash_output_chunks[1]);

    // Render the SHA-384 hash value.
    let sha384 = Paragraph::new(app.hash_generator.get_sha384())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" SHA-384 Hash ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(sha384, sha384_sha512_chunks[0]);

    // Render the SHA-512 hash value.
    let sha512 = Paragraph::new(app.hash_generator.get_sha512())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" SHA-512 Hash ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(sha512, sha384_sha512_chunks[1]);
}

// Handles the UI for number base conversion.
fn number_base_converter(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    number_base_converter_textarea: &mut TextArea,
) {
    // Split the area into two chunks: input/guide area and output area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    // Split the input/guide area into two chunks: input area and guide/status area.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Style the input text area.
    number_base_converter_textarea.set_block(
        Block::default()
            .title(" Input ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0)),
    );

    f.render_widget(&*number_base_converter_textarea, input_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(input_guide_chunks[1]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "       Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "       Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Export Generated Result",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " export/number_conversion.txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .title(" Number Base Converter Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Render the status block for messages and errors.
    let status_text = if let Some(message) = &app.number_base_converter.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };

    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Split the output area into six chunks for displaying different conversions.
    let output_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(17),
            Constraint::Percentage(16),
            Constraint::Percentage(17),
            Constraint::Percentage(16),
            Constraint::Percentage(17),
            Constraint::Percentage(17),
        ])
        .split(chunks[1]);

    // Render individual conversion outputs with styles and borders.
    let binary_to_decimal_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.binary_to_decimal.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let binary_to_decimal = Paragraph::new(binary_to_decimal_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Binary to Decimal ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(binary_to_decimal, output_chunks[0]);

    let binary_to_hexadecimal_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.binary_to_hexadecimal.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let binary_to_hexadecimal = Paragraph::new(binary_to_hexadecimal_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Binary to Hexadecimal ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(binary_to_hexadecimal, output_chunks[1]);

    let decimal_to_binary_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.decimal_to_binary.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let decimal_to_binary = Paragraph::new(decimal_to_binary_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Decimal to Binary ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(decimal_to_binary, output_chunks[2]);

    let decimal_to_hexadecimal_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.decimal_to_hexadecimal.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let decimal_to_hexadecimal = Paragraph::new(decimal_to_hexadecimal_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Decimal to Hexadecimal ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(decimal_to_hexadecimal, output_chunks[3]);

    let hexadecimal_to_binary_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.hexadecimal_to_binary.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let hexadecimal_to_binary = Paragraph::new(hexadecimal_to_binary_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Hexadecimal to Binary ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(hexadecimal_to_binary, output_chunks[4]);

    let hexadecimal_to_decimal_text = vec![Line::from(vec![Span::styled(
        app.number_base_converter.hexadecimal_to_decimal.to_string(),
        Style::default().fg(Color::Green),
    )])];
    let hexadecimal_to_decimal = Paragraph::new(hexadecimal_to_decimal_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Hexadecimal to Decimal ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(hexadecimal_to_decimal, output_chunks[5]);
}

// Handles the UI for password generator.
fn password_generator(f: &mut Frame, app: &mut App, area: Rect) {
    // Split the area into two chunks: settings/guide area and password output area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    // Create a vector of lines representing the password generator settings.
    let settings = vec![
        Line::from(vec![
            Span::raw("Length: "),
            Span::styled(
                app.password_generator.length.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Quantity: "),
            Span::styled(
                app.password_generator.quantity.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Uppercase: "),
            Span::styled(
                app.password_generator.use_uppercase.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Lowercase: "),
            Span::styled(
                app.password_generator.use_lowercase.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Numbers: "),
            Span::styled(
                app.password_generator.use_numbers.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Symbols: "),
            Span::styled(
                app.password_generator.use_symbols.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Similar Characters: "),
            Span::styled(
                app.password_generator.use_similar_characters.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Duplicate Characters: "),
            Span::styled(
                app.password_generator.use_duplicate_characters.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Sequential Characters: "),
            Span::styled(
                app.password_generator.use_sequential_characters.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    // Split the settings/guide area into two chunks: settings area and guide/status area.
    let settings_guide_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(37), Constraint::Percentage(63)])
        .split(chunks[0]);

    // Render the password generator settings.
    let settings_widget = Paragraph::new(settings)
        .block(
            Block::default()
                .title(" Settings ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_widget(settings_widget, settings_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(20)])
        .split(settings_guide_chunks[1]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "g",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate password",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "c",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Clear password",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Export Generated password",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "i",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Increase Password Length",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Decrease Password Length",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "m",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate Multiple passwords",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "k",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Increase Password Quantity",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "j",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Decrease Password Quantity",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "u",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Include Uppercase Characters",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (e.g. ABCDE)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled(
                "l",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Include Lowercase Characters",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (e.g. abcde)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled(
                "n",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Numbers", Style::default().fg(Color::White)),
            Span::styled(
                " (e.g. 12345)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Symbols", Style::default().fg(Color::White)),
            Span::styled(
                " (e.g. !@#$%^",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "z",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Similar Characters", Style::default().fg(Color::White)),
            Span::styled(
                " (e.g. i,l,L,o,0,O, etc.)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Duplicate Characters",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                " (e.g. pp, 11)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "v",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Sequential Characters",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                " (e.g. abc, 234)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " export/password.txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Warning:",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " Multi password generator isn't working",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .title(" Password Generator Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Render the status block for messages and errors.
    let status_text = if let Some(message) = &app.password_generator.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };
    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Render the generated password.
    let password = Paragraph::new(app.password_generator.generated_password.clone())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Generated Password ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(password, chunks[1]);
}

// Handles the UI for QR code generator.
fn qr_code_generator(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    qr_code_generator_textarea: &mut TextArea,
) {
    // Split the area into two chunks: input/guide area and QR code output area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    // Split the input/guide area into two chunks: input area and guide/status area.
    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[0]);

    // Style the input text area.
    qr_code_generator_textarea.set_block(
        Block::default()
            .title(" Input ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0)),
    );

    f.render_widget(&*qr_code_generator_textarea, input_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(input_guide_chunks[1]);

    // Create a guide text with shortcut keys, instructions, and file path information.
    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "        Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "        Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "    Export Generated QR Code",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   export/",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "{input_text}",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ".txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Note:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " The file name for the QR image is created by taking ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "first ten characters",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(
                " of the information you enter in the input field.",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" QR Generator Help ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Render the status block for messages and errors.
    let status_text = if let Some(message) = &app.qr_code_generator.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };
    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Render the generated QR code string.
    let qr_code = app.qr_code_generator.get_qr_string();
    let output = Paragraph::new(qr_code)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" QR Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );
    f.render_widget(output, chunks[1]);
}

// Handles the UI for UUID generator.
fn uuid_generator(f: &mut Frame, app: &mut App, area: Rect) {
    // Split the area into two chunks: settings/guide area and UUID output area.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    // Split the settings/guide area into two chunks: settings area and guide/status area.
    let settings_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[0]);

    // Render the UUID generator settings.
    let settings = vec![Line::from(vec![
        Span::raw("Number of UUID: "),
        Span::styled(
            app.uuid_generator.length.to_string(),
            Style::default().fg(Color::Yellow),
        ),
    ])];
    let settings_widget = Paragraph::new(settings)
        .block(
            Block::default()
                .title(" Settings ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_widget(settings_widget, settings_guide_chunks[0]);

    // Split the guide/status area into two chunks: guide area and status area.
    let guide_status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(settings_guide_chunks[1]);

    // Create a guide text with shortcut keys and instructions.
    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Quit",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   Switch Tools",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate single V4 UUID",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "w",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate single V7 UUID",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "m",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate multiple V4 UUIDs",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "e",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Generate multiple V7 UUIDs",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "i",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Increase number of UUIDs to generate",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Decrease number of UUIDs to generate",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     Export Generated UUIDs",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " export/uuid.txt",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Render the guide text.
    let guide = Paragraph::new(guide_text)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .title(" UUID Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, guide_status_chunks[0]);

    // Render the status block for messages and errors.
    let status_text = if let Some(message) = &app.uuid_generator.tools_export_message {
        format!("{}", message)
    } else {
        "".to_string()
    };

    let status_block = Paragraph::new(status_text)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        )
        .block(
            Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(status_block, guide_status_chunks[1]);

    // Split the UUID output area into two chunks for Version 4 and Version 7 UUIDs.
    let version4_version7_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Render the Version 4 UUID.
    let version_4_uuid = Paragraph::new(app.uuid_generator.generated_uuid_v4.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Version 4 UUID ")
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        );
    f.render_widget(version_4_uuid, version4_version7_chunks[0]);

    // Render the Version 7 UUID.
    let version_7_uuid = Paragraph::new(app.uuid_generator.generated_uuid_v7.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Version 7 UUID ")
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        );
    f.render_widget(version_7_uuid, version4_version7_chunks[1]);
}
