use std::env;

use arboard::Clipboard;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tui_textarea::TextArea;

use tuilet::opts::Opts;
use tuilet::state::State;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    let opts = Opts::from_args(env::args());
    let mut state = State::new(&opts);

    let mut terminal = ratatui::init();

    let use_clipboard = Clipboard::new().is_ok();
    let enter_to_copy = match use_clipboard {
        true => "(Enter to copy) ",
        false => "(Clipboard disabled) ",
    };
    let mut just_copied = false;

    let title_widget = Paragraph::new(format!(
        " tuilet v{} (Ctrl-C to quit, Tab to change focus) ",
        VERSION
    ));
    let mut input_widgets = [
        TextArea::default(), // input
        TextArea::default(), // font
        TextArea::default(), // flags
    ];
    let mut output_widgets = [
        Paragraph::default(), // output
        Paragraph::default(), // cmdline
    ];

    let active_widget_index_titles = [
        " Input ",
        " Font (Up/Down to select) ",
        " Flags (add '-h' to view toilet docs) ",
        &format!("─ Output {}", enter_to_copy),
        &format!("─ Command Line {}", enter_to_copy),
    ];
    let inactive_widget_index_titles = [
        " Input ",
        " Font ",
        " Flags ",
        "─ Output ",
        "─ Command Line ",
    ];

    let mut active_widget_index = 0;
    let input_widget_index: usize = 0;
    let font_widget_index: usize = 1;
    let flags_widget_index: usize = 2;
    let output_widget_index: usize = 3;
    let cmdline_widget_index: usize = 4;
    let input_widget_indexes = [input_widget_index, font_widget_index, flags_widget_index];
    let output_widget_indexes = [output_widget_index, cmdline_widget_index];
    let widget_count = 5;

    loop {
        state.input = String::from(&input_widgets[0].lines()[0]);
        state.flags = String::from(&input_widgets[2].lines()[0]);

        input_widgets[1].delete_line_by_end();
        input_widgets[1].delete_line_by_head();
        input_widgets[1].insert_str(&state.font().name);

        state.exec();

        output_widgets[0] = Paragraph::new(state.output.clone());
        output_widgets[1] = Paragraph::new(state.toilet_cmdline.clone());

        // Handle focus
        for i in 0..widget_count {
            if i == active_widget_index {
                match i {
                    0..=2 => {
                        input_widgets[i] = active_input(
                            &input_widgets[i],
                            String::from(active_widget_index_titles[i]),
                        );
                    }
                    3..=4 => {
                        output_widgets[i - 3] = active_output(
                            &output_widgets[i - 3],
                            String::from(active_widget_index_titles[i]),
                            just_copied,
                        );
                    }
                    _ => panic!("widget index out of bounds"),
                }
            } else {
                match i {
                    0..=2 => {
                        input_widgets[i] = inactive_input(
                            &input_widgets[i],
                            String::from(inactive_widget_index_titles[i]),
                        )
                    }
                    3..=4 => {
                        output_widgets[i - 3] = inactive_output(
                            &output_widgets[i - 3],
                            String::from(inactive_widget_index_titles[i]),
                        );
                    }
                    _ => panic!("widget index out of bounds"),
                }
            }
        }

        terminal.draw(|frame| {
            state.width = frame.area().width as usize;
            let cmdline_size: u16 =
                (state.toilet_cmdline.len() as f32 / state.width as f32).ceil() as u16 + 2;

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),            // title
                    Constraint::Length(3),            // input
                    Constraint::Length(3),            // flags
                    Constraint::Length(3),            // font
                    Constraint::Min(10),              // output
                    Constraint::Length(cmdline_size), // cmdline
                ])
                .split(frame.area());

            frame.render_widget(&title_widget, layout[0]);
            frame.render_widget(&input_widgets[0], layout[1]);
            frame.render_widget(&input_widgets[1], layout[2]);
            frame.render_widget(&input_widgets[2], layout[3]);
            frame.render_widget(&output_widgets[0], layout[4]);
            frame.render_widget(&output_widgets[1], layout[5]);
        })?;

        // Handle input
        just_copied = false;
        let evt = event::read()?;
        if let Event::Key(key) = evt {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            } else if key.code == KeyCode::Tab || key.code == KeyCode::BackTab {
                // tabbing between fields
                let mut inc = 1;
                if key.modifiers.contains(KeyModifiers::SHIFT) || key.code == KeyCode::BackTab {
                    inc = widget_count - 1;
                }
                active_widget_index = (active_widget_index + inc) % widget_count;
            } else if active_widget_index == font_widget_index {
                // only accept up/down in the font widget
                if key.code == KeyCode::Up {
                    state.prev_font();
                } else if key.code == KeyCode::Down {
                    state.next_font();
                }
            } else if output_widget_indexes.contains(&active_widget_index) {
                if key.code == KeyCode::Enter && use_clipboard {
                    let mut clipboard = Clipboard::new().expect("clipboard failed");
                    if active_widget_index == output_widget_index {
                        clipboard
                            .set_text(state.toilet_cmdline_output.clone())
                            .expect("clipboard failed");
                    } else if active_widget_index == cmdline_widget_index {
                        clipboard
                            .set_text(state.toilet_cmdline.clone())
                            .expect("clipboard failed");
                    }
                    just_copied = true;
                }
            } else if input_widget_indexes.contains(&active_widget_index) {
                if key.code == KeyCode::Enter {
                    // ensure textarea contents are only one line high
                    continue;
                } else {
                    input_widgets[active_widget_index].input(evt);
                }
            }
        }
    }

    ratatui::restore();
    println!("Last command line:\n{}", state.toilet_cmdline);
    Ok(())
}

fn inactive_input<'a>(textarea: &TextArea<'a>, title: String) -> TextArea<'a> {
    let mut ta = textarea.clone();
    ta.set_cursor_line_style(Style::default());
    ta.set_cursor_style(Style::default());
    ta.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(title),
    );
    ta
}

fn active_input<'a>(textarea: &TextArea<'a>, title: String) -> TextArea<'a> {
    let mut ta = textarea.clone();
    ta.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    ta.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    ta.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bold())
            .title(title),
    );
    ta
}

fn inactive_output<'a>(paragraph: &Paragraph<'a>, title: String) -> Paragraph<'a> {
    paragraph.clone().wrap(Wrap { trim: false }).block(
        Block::new()
            .borders(Borders::TOP)
            .style(Style::default().fg(Color::Gray))
            .title(title),
    )
}

fn active_output<'a>(paragraph: &Paragraph<'a>, title: String, just_copied: bool) -> Paragraph<'a> {
    let mut t = title.clone();
    if just_copied {
        t.push_str("Copied! ");
    }
    paragraph.clone().wrap(Wrap { trim: false }).block(
        Block::new()
            .borders(Borders::TOP)
            .style(Style::default().bold())
            .title(t),
    )
}
