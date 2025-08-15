use crate::app::App;

pub mod app;
pub mod event;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(draw)?;
//         if let Event::Key(key) = event::read()? {
//             if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                 break Ok(());
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::widgets::Padding;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_render_app() {
        // Given the app is running
        let app = App::default();
        // When the application is open
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        // Then the snapshot is accurate
        assert_snapshot!(terminal.backend());
    }
}
