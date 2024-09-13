use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use state::State;
use tui_textarea::TextArea;

mod fonts;
mod state;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FONT_WIDGET_INDEX: usize = 1;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = match State::init() {
        Ok(state) => state,
        Err(e) => panic!("could not start tuilet: {}", e),
    };

    let mut input_widgets = [
        TextArea::default(), // input
        TextArea::default(), // font
        TextArea::default(), // flags
    ];

    let input_widget_titles = [" Input ", " Font (select with up/down arrow) ", " Flags "];

    let mut output_widget: Paragraph;
    let mut cmdline_widget: Paragraph;

    let mut active_widget = 0;

    loop {
        state.input = String::from(&input_widgets[0].lines()[0]);
        state.flags = String::from(&input_widgets[2].lines()[0]);
        state.exec();

        input_widgets[FONT_WIDGET_INDEX].delete_line_by_end();
        input_widgets[FONT_WIDGET_INDEX].delete_line_by_head();
        input_widgets[FONT_WIDGET_INDEX].insert_str(&state.font().name);

        output_widget = Paragraph::new(state.output.clone());
        cmdline_widget = Paragraph::new(state.toilet_cmdline.clone());

        terminal.draw(|frame| {
            state.width = frame.area().width as usize;

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2), // title
                    Constraint::Length(3), // input
                    Constraint::Length(3), // flags
                    Constraint::Length(3), // font
                    Constraint::Min(10),   // output
                    Constraint::Length(3), // cmdline
                ])
                .split(frame.area());

            let title = format!(" tuilet v{} (press Ctrl-C to quit) ", VERSION);

            let output_block = output_widget.block(
                Block::bordered()
                    .style(Style::default().fg(Color::Gray))
                    .title(" Output "),
            );

            let cmdline_block = cmdline_widget.block(
                Block::default()
                    .style(Style::default().fg(Color::Gray))
                    .title("  Command line "),
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
            frame.render_widget(cmdline_block, layout[5])
        })?;

        let evt = event::read()?;
        if let Event::Key(key) = evt {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            } else if key.code == KeyCode::Enter {
                // ensure textarea contents are only one line high
                continue;
            } else if key.code == KeyCode::Tab || key.code == KeyCode::BackTab {
                // tabbing between fields
                let len = input_widgets.len();
                let mut inc = 1;
                if key.modifiers.contains(KeyModifiers::SHIFT) || key.code == KeyCode::BackTab {
                    inc = len - 1;
                }
                active_widget = (active_widget + inc) % len;
            } else if active_widget == FONT_WIDGET_INDEX {
                // only accept up/down in the font widget
                if key.code == KeyCode::Up {
                    state.prev_font();
                } else if key.code == KeyCode::Down {
                    state.next_font();
                }
            } else {
                input_widgets[active_widget].input(evt);
            }
        }
    }
    ratatui::restore();
    print!("{}", state.toilet_cmdline);
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
