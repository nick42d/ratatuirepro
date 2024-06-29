use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
    Terminal,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut styles = [
        Style::new().red(),
        Style::new(),
        Style::new().bold(),
        Style::new().green(),
    ]
    .into_iter()
    .cycle();
    loop {
        terminal.clear()?;
        terminal.draw(|frame| {
            let area = frame.size();
            let style = styles.next().unwrap();
            let text = "Text\0".to_string();
            let w = Paragraph::new(text)
                .style(style)
                .block(Block::bordered().title("Press q to quit"));
            frame.render_widget(w, area);
        })?;
        if event::poll(std::time::Duration::from_millis(500))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
