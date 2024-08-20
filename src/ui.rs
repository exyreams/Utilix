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

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    base64_converter_textarea: &mut TextArea,
    date_converter_textarea: &mut TextArea,
    hash_generator_textarea: &mut TextArea,
    qr_code_generator_textarea: &mut TextArea,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            ui(
                f,
                &mut app,
                base64_converter_textarea,
                date_converter_textarea,
                hash_generator_textarea,
                qr_code_generator_textarea,
            )
        })?;

        match crossterm::event::read()? {
            crossterm::event::Event::Key(key) => {
                let input = match key.code {
                    KeyCode::Char(c) => Input {
                        key: Key::Char(c),
                        ..Default::default()
                    },

                    KeyCode::Esc => Input {
                        key: Key::Esc,
                        ..Default::default()
                    },

                    KeyCode::Tab => {
                        app.current_tool = match app.current_tool {
                            Tool::Base64Encoder => Tool::DateConverter,
                            Tool::DateConverter => Tool::HashGenerator,
                            Tool::HashGenerator => Tool::PasswordGenerator,
                            Tool::PasswordGenerator => Tool::QRCodeGenerator,
                            Tool::QRCodeGenerator => Tool::UuidGenerator,
                            Tool::UuidGenerator => Tool::Base64Encoder,
                        };
                        continue;
                    }

                    KeyCode::Left => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::WordBack);
                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::WordBack);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordBack);
                        } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                            
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);

                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);
                           
                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);
                            
                            qr_code_generator_textarea.move_cursor(CursorMove::Head);
                            qr_code_generator_textarea.start_selection();
                            qr_code_generator_textarea.move_cursor(CursorMove::End);
                        } else {
                            base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            base64_converter_textarea.cancel_selection();
                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            date_converter_textarea.cancel_selection();
                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            hash_generator_textarea.cancel_selection();
                            qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Back);
                            qr_code_generator_textarea.cancel_selection();
                        }
                        continue;
                    }

                    KeyCode::Right => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            date_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            hash_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::WordForward);
                        } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                            
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);
                            
                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);
                            
                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);
                            
                            qr_code_generator_textarea.move_cursor(CursorMove::Head);
                            qr_code_generator_textarea.start_selection();
                            qr_code_generator_textarea.move_cursor(CursorMove::End);
                        } else {
                            base64_converter_textarea
                                .move_cursor(tui_textarea::CursorMove::Forward);
                            base64_converter_textarea.cancel_selection();
                            date_converter_textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            date_converter_textarea.cancel_selection();
                            hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            hash_generator_textarea.cancel_selection();
                            qr_code_generator_textarea
                                .move_cursor(tui_textarea::CursorMove::Forward);
                            qr_code_generator_textarea.cancel_selection();
                        }
                        continue;
                    }

                    KeyCode::Up => {
                        base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        date_converter_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Up);
                        qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Up);

                        continue;
                    }

                    KeyCode::Down => {
                        base64_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        date_converter_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        hash_generator_textarea.move_cursor(tui_textarea::CursorMove::Down);
                        qr_code_generator_textarea.move_cursor(tui_textarea::CursorMove::Down);

                        continue;
                    }

                    KeyCode::Enter => {
                        base64_converter_textarea.insert_newline();
                        date_converter_textarea.insert_newline();
                        hash_generator_textarea.insert_newline();
                        qr_code_generator_textarea.insert_newline();
                        continue;
                    }

                    KeyCode::Backspace => {
                        base64_converter_textarea.delete_char();
                        date_converter_textarea.delete_char();
                        hash_generator_textarea.delete_char();
                        qr_code_generator_textarea.delete_char();
                        continue;
                    }

                    KeyCode::Delete => {
                        base64_converter_textarea.delete_next_char();
                        date_converter_textarea.delete_next_char();
                        hash_generator_textarea.delete_next_char();
                        qr_code_generator_textarea.delete_next_char();
                        qr_code_generator_textarea.insert_newline();
                        continue;
                    }
                    _ => continue,
                };

                match input.key {
                    Key::Esc => return Ok(()),
                    
                    Key::Char(c) => match app.current_tool {
                        Tool::Base64Encoder => {

                            if !key.modifiers.contains(KeyModifiers::ALT)
                            && !key.modifiers.contains(KeyModifiers::SHIFT)
                            && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                base64_converter_textarea.insert_char(c);
                            }
                            
                            if base64_converter_textarea.lines().join("\n").len() % 84 == 0 {
                                base64_converter_textarea.insert_newline();
                            }

                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'e' {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.encode();
                            } else if key.modifiers.contains(KeyModifiers::ALT) && c == 'd' {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.decode();
                            } else if key.modifiers.contains(KeyModifiers::ALT) && c == 'x'{
                                let _ = app.base64_encoder.write_to_file();
                            } 
                            else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("");
                                app.base64_encoder.encode();
                                app.base64_encoder.decode();
                            }
                        }

                        Tool::DateConverter => {
                            date_converter_textarea.insert_char(c);
                        
                            if date_converter_textarea.lines().join("\n").len() % 50 == 0 {
                                date_converter_textarea.insert_newline();
                            }
                        
                            app.date_converter.input = date_converter_textarea.lines().join("\n");
                            app.date_converter.convert_all();
                        }

                        Tool::HashGenerator => {
                            if !key.modifiers.contains(KeyModifiers::ALT)
                            && !key.modifiers.contains(KeyModifiers::SHIFT)
                            && !key.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                hash_generator_textarea.insert_char(c);
                            }
                            
                            if hash_generator_textarea.lines().join("\n").len() % 62 == 0 {
                                hash_generator_textarea.insert_newline();
                            }

                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'x' {
                                let _ = app.hash_generator.write_to_file();
                            } else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            {
                                let new_input = hash_generator_textarea.lines().join("\n");
                                app.hash_generator.update_input(&new_input);
                            }
                        }
                       
                        Tool::PasswordGenerator => {
                            match c {
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
                                'x' => {
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
                                _ => {}
                            }
                        }

                        Tool::QRCodeGenerator => {
                            qr_code_generator_textarea.insert_char(c);

                            if base64_converter_textarea.lines().join("\n").len() % 68 == 0 {
                                base64_converter_textarea.insert_newline();
                            }

                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'q' {
                                app.qr_code_generator.input =
                                    qr_code_generator_textarea.lines().join("\n");
                                app.qr_code_generator.generate_qr_code();
                            } else if !key.modifiers.contains(KeyModifiers::ALT) {
                                app.qr_code_generator.input =
                                    qr_code_generator_textarea.lines().join("\n");
                                app.qr_code_generator.generate_qr_code();
                            }
                        }

                        Tool::UuidGenerator => {
                            match c {
                                's' => {
                                    app.uuid_generator.generate_v4_uuid();
                                }
                                'm' => {
                                    app.uuid_generator.generate_multiple_v4_uuids();
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
                                'x' => {
                                    let _ = app.uuid_generator.write_to_file();
                                }
                                _ => {}
                            }
                        }
                        
                    },
                    _ => {}
                }
            }
            _ => continue,
        }
    }
}

fn ui(
    f: &mut Frame,
    app: &mut App,
    base64_converter_textarea: &mut TextArea,
    date_converter_textarea: &mut TextArea,
    hash_generator_textarea: &mut TextArea,
    qr_code_generator_textarea: &mut TextArea,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.area());

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

    let tabs = Tabs::new(vec![
        Span::raw("Base64 Encoder"),
        Span::raw("Date Converter"),
        Span::raw("Hash Generator"),
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
                    .fg(Color::Magenta),
            )
            .border_type(BorderType::Rounded),
    )
    .style(Style::default())
    .highlight_style(Style::default().bg(Color::LightMagenta))
    .select(match app.current_tool {
        Tool::Base64Encoder => 0,
        Tool::DateConverter => 1,
        Tool::HashGenerator => 2,
        Tool::PasswordGenerator => 3,
        Tool::QRCodeGenerator => 4,
        Tool::UuidGenerator => 5,
    })
    .divider("|")
    .padding(" ", " ");
    f.render_widget(tabs, chunks[1]);

    let tool_content_area = chunks[2];

    match app.current_tool {
        Tool::Base64Encoder => {
            render_base64_encoder(f, app, tool_content_area, base64_converter_textarea)
        }
        Tool::DateConverter => {
            render_date_converter(f, app, tool_content_area, date_converter_textarea)
        }
        Tool::HashGenerator => {
            render_hash_generator(f, app, tool_content_area, hash_generator_textarea)
        }
        Tool::PasswordGenerator => render_password_generator(f, app, tool_content_area),
        Tool::QRCodeGenerator => {
            render_qr_code_generator(f, app, tool_content_area, qr_code_generator_textarea)
        }
        Tool::UuidGenerator => render_uuid_generator(f, app, tool_content_area),
    }
}

fn render_base64_encoder(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    base64_converter_textarea: &mut TextArea,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50), 
        ])
        .split(chunks[0]);

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

    f.render_widget(&*base64_converter_textarea, input_guide_chunks[0]);

    let guide_text = vec![
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("             Quit", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "Shift + 🡸 /🡺 ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Selection", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + e",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Encode", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Decode", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("         Export Generated Hash", Style::default().fg(Color::White)),
        ]), 
        Line::from(vec![Span::raw("")]), 
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   export/base64encoder.txt", Style::default().fg(Color::White)),
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
                    
            ),
        ]), 
    ];

    let guide = Paragraph::new(guide_text)
        .block(
            Block::default()
                .title(" Base64 Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, input_guide_chunks[1]);

    let encoded_decoded_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

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

fn render_date_converter(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    date_converter_textarea: &mut TextArea,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(chunks[0]);

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

    f.render_widget(&*date_converter_textarea, input_guide_chunks[0]);
    
    let guide_text = vec![        
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("    Quit", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Supported Formats:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "YYYY-MM-DD H:M:S",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "YYYY-MM-DDTH:M:S:z",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "YYYY-MM-DD",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "DD/MM/YYYY H:M:S",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "DD/MM/YYYY  ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Examples:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        
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
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Time Only:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 13:00:00 or 7:30:00 AM",
                Style::default()
                    .fg(Color::White)
            ),
        ]),
    ];

    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" Date Converter Help ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(guide, input_guide_chunks[1]);

    let converstion_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[1]);
    
    let converstion_chunks_first_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(converstion_chunks[0]);
    
    let converstion_chunks_second_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(converstion_chunks_first_split[0]);
    
    let rfc3339_text = vec![
        Line::from(vec![Span::styled(app.date_converter.rfc3339.to_string(), Style::default().fg(Color::Green))]),
        
    ];
    
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
    
    let rfc2822_text = vec![
        Line::from(vec![Span::styled(app.date_converter.rfc2822.to_string(), Style::default().fg(Color::Green))]),
        
    ];
    
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
     
    let converstion_chunks_third_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        
        .split(converstion_chunks[1]);
    
    let converstion_chunks_third_split_half = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    
    .split(converstion_chunks_third_split[0]);

    let timeonly_text = vec![
        Line::from(vec![Span::styled(app.date_converter.time_only.to_string(), Style::default().fg(Color::Green))]),
        
    ];

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

    let iso8601_text = vec![
        Line::from(vec![Span::styled(app.date_converter.iso8601.to_string(), Style::default().fg(Color::Green))]),
        
    ];

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
    
    let unixtimestamp_text = vec![
        Line::from(vec![Span::styled(app.date_converter.unix_timestamp.to_string(), Style::default().fg(Color::Green))]),
        
    ];

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
    
    let converstion_chunks_fourth_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(converstion_chunks_first_split[1]);
    
    let humanreadable_text = vec![
        Line::from(vec![Span::styled(app.date_converter.human_readable.to_string(), Style::default().fg(Color::Green))]),
        
    ];
    
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
    
    let shordate_text = vec![
        Line::from(vec![Span::styled(app.date_converter.short_date.to_string(), Style::default().fg(Color::Green))]),
        
    ];
    
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

fn render_hash_generator(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    hash_generator_textarea: &mut TextArea,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[0]);

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

    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("       Quit", Style::default().fg(Color::White)),
        ]), 
        Line::from(vec![
            Span::styled(
                "Alt + x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Export Generated Hash", Style::default().fg(Color::White)),
        ]), 
        Line::from(vec![Span::raw("")]), 
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" export/hashgenerator.txt", Style::default().fg(Color::White)),
        ]),
    ];

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
    f.render_widget(guide, input_guide_chunks[1]);
    
    let hash_output_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks[1]);
    
    let sha1_sha256_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(hash_output_chunks[0]);

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

    let sha384_sha512_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(hash_output_chunks[1]);

    let sha384 = Paragraph::new(app.hash_generator.get_sha384())
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
    f.render_widget(sha384, sha384_sha512_chunks[0]);

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

fn render_password_generator(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

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

    let settings_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(37), 
            Constraint::Percentage(63), 
        ])
        .split(chunks[0]);

    let settings_widget = Paragraph::new(settings)
        .block(
            Block::default()
                .title(" Settings ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    f.render_widget(settings_widget, settings_guide_chunks[0]);

    let guide_text = vec![
        
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Quit", Style::default().fg(Color::White)),
        ]),
        
        Line::from(vec![
            Span::styled(
                "g",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Generate password", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "c",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Clear password", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "i",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Increase Password Length", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Decrease Password Length", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "m",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Generate Multiple passwords", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "k",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Increase Password Quantity", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "j",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Decrease Password Quantity", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "u",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Uppercase Characters", Style::default().fg(Color::White)),
            Span::styled(" (e.g. ABCDEFGH)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "l",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Lowercase Characters", Style::default().fg(Color::White)),
            Span::styled(" (e.g. abcdefgh)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "n",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Numbers", Style::default().fg(Color::White)),
            Span::styled(" (e.g. 1234567890)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Include Symbols", Style::default().fg(Color::White)),
            Span::styled(" (e.g. !@#$%^&*()_+-=[]{}|;:,.<>?)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "z",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Similar Characters", Style::default().fg(Color::White)),
            Span::styled(" (e.g. i, l, L, o, 0, O, etc.)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Duplicate Characters", Style::default().fg(Color::White)),
            Span::styled(" (e.g. aa, bb, 11)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "v",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Sequential Characters", Style::default().fg(Color::White)),
            Span::styled(" (e.g. abc, def, 234, 567)", Style::default().fg(Color::Cyan))
        ]),
        Line::from(vec![
            Span::styled(
                "Warning:",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Multi password generator isn't working", Style::default().fg(Color::White)),
            
        ]),
        
    ];

    let guide = Paragraph::new(guide_text)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .title(" Password Generator Help ")
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        );
    f.render_widget(guide, settings_guide_chunks[1]);

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
                .padding(Padding::new(1, 1, 0, 0))
        )
        .wrap(Wrap { trim: true });
        
    f.render_widget(password, chunks[1]);
}

fn render_qr_code_generator(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    qr_code_generator_textarea: &mut TextArea,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50),
        ])
        .split(chunks[0]);

    qr_code_generator_textarea.set_block(
        Block::default()
            .title(" Input ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    f.render_widget(&*qr_code_generator_textarea, input_guide_chunks[0]);

    let guide_text = vec![
        Line::from(vec![Span::raw("Controls:")]),
        Line::from(vec![Span::raw("Esc: Quit Program")]),
        Line::from(vec![Span::raw("  - 'g': Generate QR Code")]),
        Line::from(vec![Span::raw("  - Backspace: Clear input")]),
    ];

    let guide = Paragraph::new(guide_text)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
        .block(
            Block::default()
                .title(" Guide ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 0, 0, 0)),
        );
    f.render_widget(guide, input_guide_chunks[1]);

    let output = Paragraph::new(app.qr_code_generator.qr_code.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" QR Code ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(output, chunks[1]);
}

fn render_uuid_generator(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

        let settings_guide_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ])
            .split(chunks[0]);
    

        let settings = vec![
            Line::from(vec![
                Span::raw("Number of UUID: "),
                Span::styled(
                    app.uuid_generator.length.to_string(),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
        ];

        let settings_widget = Paragraph::new(settings)
            .block(
                Block::default()
                    .title(" Settings ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .wrap(Wrap { trim: true })
            .scroll((0, 0))
            .style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            );
    
        f.render_widget(settings_widget, settings_guide_chunks[0]);

    

    let guide_text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Quit", Style::default().fg(Color::White)),
        ]),
        
        Line::from(vec![
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Generate single UUID", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "m",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Generate multiple UUIDs", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "i",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Increase number of UUIDs", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "d",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Decrease number of UUIDs", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "x",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("     Export Generated UUIDs", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled(
                "Exported File Path:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" export/uuidgenerator.txt", Style::default().fg(Color::White)),
        ]),
    ];

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
    f.render_widget(guide, settings_guide_chunks[1]);

    let version_4_uuid = Paragraph::new(app.uuid_generator.generated_uuid.as_str())
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
                .padding(Padding::new(1, 0, 0, 0))
        );
    f.render_widget(version_4_uuid, chunks[1]);

}
