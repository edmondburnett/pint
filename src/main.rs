use color_eyre::owo_colors::OwoColorize;
// use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Rect},
    prelude::Layout,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Gauge, Paragraph, Widget},
};
use std::io;

mod vertical_gauge;
use vertical_gauge::VerticalGauge;

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
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(frame.area());

        let meter = Meter::new(self.water_amount, self.water_goal);

        frame.render_widget(&meter, layout[0]);
        frame.render_widget(self, layout[1]);
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
            KeyCode::Char('d') => self.drink(8),
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
        let title = Line::from(" Dram ".bold());
        let instructions = Line::from(vec![
            " Drink ".into(),
            "<d>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.left_aligned())
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

#[derive(Debug, Default)]
pub struct Meter {
    water_amount: u16,
    water_goal: u16,
}

impl Meter {
    fn new(water_amount: u16, water_goal: u16) -> Self {
        Self {
            water_amount,
            water_goal,
        }
    }
}

impl Widget for &Meter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let percentage = ((self.water_amount as f64 / self.water_goal as f64) * 100.0) as u16;
        let clamped_percentage = percentage.clamp(u16::MIN, 100);
        // let title = Line::from("Meter");
        let title = Line::from(vec![
            self.water_amount.to_string().yellow(),
            "/".into(),
            self.water_goal.to_string().yellow(),
            " oz".into(),
        ]);

        VerticalGauge::default()
            .block(Block::bordered().title(title.left_aligned()))
            .gauge_style(Style::new().blue().italic())
            .label(percentage.to_string() + "%")
            .percent(clamped_percentage)
            .render(area, buf);
    }
}
