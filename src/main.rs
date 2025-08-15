// use crate::event::{AppEvent, Event, EventHandler};
// use ratatui::{
//     DefaultTerminal,
//     crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
// };
// 
// pub mod app;
// pub mod event;
// pub mod ui;
// 
// /// Application.
// #[derive(Debug)]
// pub struct App {
//     /// Is the application running?
//     pub running: bool,
//     /// Counter.
//     pub counter: u8,
//     /// Event handler.
//     pub events: EventHandler,
// }
// 
// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             counter: 0,
//             events: EventHandler::new(),
//         }
//     }
// }
// 
// impl App {
//     /// Constructs a new instance of [`App`].
//     pub fn new() -> Self {
//         Self::default()
//     }
// 
//     /// Run the application's main loop.
//     pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
//         while self.running {
//             terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
//             self.handle_events()?;
//         }
//         Ok(())
//     }
// 
//     pub fn handle_events(&mut self) -> color_eyre::Result<()> {
//         match self.events.next()? {
//             Event::Tick => self.tick(),
//             Event::Crossterm(event) => match event {
//                 crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
//                 _ => {}
//             },
//             Event::App(app_event) => match app_event {
//                 AppEvent::Increment => self.increment_counter(),
//                 AppEvent::Decrement => self.decrement_counter(),
//                 AppEvent::Quit => self.quit(),
//             },
//         }
//         Ok(())
//     }
// 
//     /// Handles the key events and updates the state of [`App`].
//     pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
//         match key_event.code {
//             KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
//             KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
//                 self.events.send(AppEvent::Quit)
//             }
//             KeyCode::Right => self.events.send(AppEvent::Increment),
//             KeyCode::Left => self.events.send(AppEvent::Decrement),
//             // Other handlers you could add here.
//             _ => {}
//         }
//         Ok(())
//     }
// 
//     /// Handles the tick event of the terminal.
//     ///
//     /// The tick event is where you can update the state of your application with any logic that
//     /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
//     pub fn tick(&self) {}
// 
//     /// Set running to false to quit the application.
//     pub fn quit(&mut self) {
//         self.running = false;
//     }
// 
//     pub fn increment_counter(&mut self) {
//         self.counter = self.counter.saturating_add(1);
//     }
// 
//     pub fn decrement_counter(&mut self) {
//         self.counter = self.counter.saturating_sub(1);
//     }
// }
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
