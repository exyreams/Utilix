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
                            // Selection for Base64 Converter
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);

                            // Selection for Date Converter
                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);
                            // Selection for Hash Generator
                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);
                            // Selection for QR Generator
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
                            // Selection for Base64 Converter
                            base64_converter_textarea.move_cursor(CursorMove::Head);
                            base64_converter_textarea.start_selection();
                            base64_converter_textarea.move_cursor(CursorMove::End);
                            // Selection for Date Converter
                            date_converter_textarea.move_cursor(CursorMove::Head);
                            date_converter_textarea.start_selection();
                            date_converter_textarea.move_cursor(CursorMove::End);
                            // Selection for Hash Generator
                            hash_generator_textarea.move_cursor(CursorMove::Head);
                            hash_generator_textarea.start_selection();
                            hash_generator_textarea.move_cursor(CursorMove::End);
                            // Selection for QR Generator
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
                            base64_converter_textarea.insert_char(c);

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
                            } else if !key.modifiers.contains(KeyModifiers::ALT)
                                && !key.modifiers.contains(KeyModifiers::SHIFT)
                            {
                                app.base64_encoder.input =
                                    base64_converter_textarea.lines().join("\n");
                                app.base64_encoder.encode();
                                app.base64_encoder.decode();
                            }
                        }

                        Tool::DateConverter => {
                            date_converter_textarea.insert_char(c);

                            if base64_converter_textarea.lines().join("\n").len() % 50 == 0 {
                                base64_converter_textarea.insert_newline();
                            }

                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'd' {
                                app.date_converter.input =
                                    date_converter_textarea.lines().join("\n");
                                app.date_converter.convert_date();
                            } else if !key.modifiers.contains(KeyModifiers::ALT) {
                                app.date_converter.input =
                                    date_converter_textarea.lines().join("\n");
                                app.date_converter.convert_date();
                            }
                        }

                        Tool::HashGenerator => {
                            hash_generator_textarea.insert_char(c);

                            if base64_converter_textarea.lines().join("\n").len() % 68 == 0 {
                                base64_converter_textarea.insert_newline();
                            }

                            if key.modifiers.contains(KeyModifiers::ALT) && c == 'h' {
                                app.hash_generator.input =
                                    hash_generator_textarea.lines().join("\n");
                                app.hash_generator.generate_hash();
                            } else if !key.modifiers.contains(KeyModifiers::ALT) {
                                app.hash_generator.input =
                                    hash_generator_textarea.lines().join("\n");
                                app.hash_generator.generate_hash();
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
                        _ => {}
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
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Input
            Constraint::Percentage(50), // Guide
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
        Line::from(vec![Span::raw("")]),
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
                "Backspace",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("       Clear input", Style::default().fg(Color::White)),
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
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let input_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Input
            Constraint::Percentage(50), // Guide
        ])
        .split(chunks[0]);

    date_converter_textarea.set_block(
        Block::default()
            .title(" Enter Date ")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded),
    );

    date_converter_textarea.set_style(Style::default().bold());

    f.render_widget(&*date_converter_textarea, input_guide_chunks[0]);

    let guide_text = vec![
        Line::from(vec![Span::raw("Controls:")]),
        Line::from(vec![Span::raw("Esc: Quit Program")]),
        Line::from(vec![Span::raw("  - 'c': Convert Date")]),
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

    let output = Paragraph::new(app.date_converter.output.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Converted Date ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(output, chunks[1]);
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
            Constraint::Percentage(50), // Input
            Constraint::Percentage(50), // Guide
        ])
        .split(chunks[0]);

    hash_generator_textarea.set_block(
        Block::default()
            .title(" Input ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    f.render_widget(&*hash_generator_textarea, input_guide_chunks[0]);

    let guide_text = vec![
        Line::from(vec![Span::raw("Controls:")]),
        Line::from(vec![Span::raw("Esc: Quit Program")]),
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

    let output = Paragraph::new(app.hash_generator.hash.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .title(" Hash ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(output, chunks[1]);
}

fn render_password_generator(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let settings = vec![
        Line::from(vec![Span::raw("Password Generator Options:")]),
        Line::from(vec![
            Span::raw("Length: "),
            Span::styled(
                app.password_generator.length.to_string(),
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
    ];

    let settings_guide_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Settings
            Constraint::Percentage(50), // Guide
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
        Line::from(vec![Span::raw("Controls:")]),
        Line::from(vec![Span::raw("Esc: Quit Program")]),
        Line::from(vec![Span::raw("  - 'd': Decode")]),
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
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
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
            Constraint::Percentage(50), // Input
            Constraint::Percentage(50), // Guide
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

    let guide_text = vec![
        Line::from(vec![Span::raw("Controls:")]),
        Line::from(vec![Span::raw("Esc: Quit Program")]),
        Line::from(vec![Span::raw("  - 'd': Decode")]),
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
    f.render_widget(guide, chunks[0]);

    let output = Paragraph::new(app.uuid_generator.generated_uuid.as_str())
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Generated UUID ")
                .border_type(BorderType::Rounded),
        );
    f.render_widget(output, chunks[1]);
}
