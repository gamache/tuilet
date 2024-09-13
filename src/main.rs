use std::process::Command;

use ratatui::{
    crossterm::event,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::{Input, Key, TextArea};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let mut input_widgets = [
        TextArea::default(), // input
        TextArea::default(), // flags
        TextArea::default(), // font
    ];

    let input_widget_titles = [" Input ", " Flags ", " Font "];

    let mut output: Paragraph;

    let mut active_widget = 0;

    loop {
        let input_text = &input_widgets[0].lines()[0];
        let cmd_output = Command::new("toilet")
            .args([input_text])
            .output()
            .expect("nope");
        let output_text = String::from_utf8(cmd_output.stdout).expect("whoops");
        output = Paragraph::new(output_text);

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2), // title
                    Constraint::Length(3), // input
                    Constraint::Length(3), // flags
                    Constraint::Length(3), // font
                    Constraint::Min(10),   // output
                ])
                .split(frame.area());

            let title = format!(" tuilet v{} (press Ctrl-C to quit) ", VERSION);

            let output_block = output.block(
                Block::bordered()
                    .style(Style::default().fg(Color::Gray))
                    .title(" Output "),
            );

            for i in 0..(input_widgets.len()) {
                if i == active_widget {
                    activate(&mut input_widgets[i], input_widget_titles[i]);
                } else {
                    inactivate(&mut input_widgets[i], input_widget_titles[i]);
                }
            }

            frame.render_widget(Paragraph::new(title), layout[0]);
            frame.render_widget(&input_widgets[0], layout[1]);
            frame.render_widget(&input_widgets[1], layout[2]);
            frame.render_widget(&input_widgets[2], layout[3]);
            frame.render_widget(output_block, layout[4]);
        })?;

        match event::read()?.into() {
            Input {
                key: Key::Char('c'),
                ctrl: true,
                ..
            } => break,
            Input {
                key: Key::Enter, ..
            } => continue,
            Input { key: Key::Tab, .. } => {
                active_widget = (active_widget + 1) % input_widgets.len();
            }
            event => {
                input_widgets[active_widget].input(event);
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn inactivate(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(String::from(title)),
    );
}

fn activate(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(String::from(title)),
    );
}
