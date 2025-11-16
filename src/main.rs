// use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    water_amount: u16,
    water_goal: u16,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.water_goal = 128;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // event::read blocks until there is an event. switch to event::poll later if you need to perform other tasks
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_event(key_event),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('d') => self.drink(16),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn drink(&mut self, amount: u16) {
        self.water_amount = self.water_amount.saturating_add(amount);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Pint ".bold());
        let instructions = Line::from(vec![
            " Drink ".into(),
            "<d>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::PLAIN);

        let water_counter_text = Text::from(vec![Line::from(vec![
            "Oz: ".into(),
            self.water_amount.to_string().yellow(),
            "/".into(),
            self.water_goal.to_string().yellow(),
        ])]);

        Paragraph::new(water_counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
