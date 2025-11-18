use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Block, Widget},
};

/// A ratatui widget to display a vertical progress bar that fills from bottom to top.
/// Based on the original Gauge widget.
///
/// # Example
///
/// ```
/// use ratatui::{
///     style::{Color, Style},
///     widgets::Block,
/// };
///
/// VerticalGauge::default()
///     .block(Block::bordered().title("Progress"))
///     .gauge_style(Style::default().fg(Color::Blue).bg(Color::LightBlue))
///     .percent(43);
/// ```

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VerticalGauge<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<Span<'a>>,
    use_unicode: bool,
    style: Style,
    gauge_style: Style,
}

impl<'a> VerticalGauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Self {
        assert!(percent <= 100, "Percentage should be between 0 and 100 (inclusive)");
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    pub fn ratio(mut self, ratio: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&ratio),
            "Ratio should be between 0.0 and 1.0 (inclusive)"
        );
        self.ratio = ratio;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn use_unicode(mut self, use_unicode: bool) -> Self {
        self.use_unicode = use_unicode;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, gauge_style: Style) -> Self {
        self.gauge_style = gauge_style;
        self
    }
}

impl Widget for VerticalGauge<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &VerticalGauge<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        if let Some(ref block) = self.block {
            let inner = block.inner(area);
            block.clone().render(area, buf);
            self.render_gauge(inner, buf);
        } else {
            self.render_gauge(area, buf);
        }
    }
}

impl VerticalGauge<'_> {
    fn render_gauge(&self, gauge_area: Rect, buf: &mut Buffer) {
        if gauge_area.is_empty() {
            return;
        }

        buf.set_style(gauge_area, self.gauge_style);

        let default_label = Span::raw(format!("{}%", f64::round(self.ratio * 100.0)));
        let label = self.label.as_ref().unwrap_or(&default_label);
        let clamped_label_width = gauge_area.width.min(label.width() as u16);
        let label_col = gauge_area.left() + (gauge_area.width - clamped_label_width) / 2;
        let label_row = gauge_area.top() + gauge_area.height / 2;

        let filled_height = f64::from(gauge_area.height) * self.ratio;
        let end = if self.use_unicode {
            gauge_area.bottom() - filled_height.floor() as u16
        } else {
            gauge_area.bottom() - filled_height.round() as u16
        };

        for y in end..gauge_area.bottom() {
            for x in gauge_area.left()..gauge_area.right() {
                if x < label_col || x >= label_col + clamped_label_width || y != label_row {
                    buf[(x, y)]
                        .set_symbol(symbols::block::FULL)
                        .set_fg(self.gauge_style.fg.unwrap_or(Color::Reset))
                        .set_bg(self.gauge_style.bg.unwrap_or(Color::Reset));
                } else {
                    buf[(x, y)]
                        .set_symbol(" ")
                        .set_fg(self.gauge_style.bg.unwrap_or(Color::Reset))
                        .set_bg(self.gauge_style.fg.unwrap_or(Color::Reset));
                }
            }
        }

        if self.use_unicode && self.ratio < 1.0 {
            for x in gauge_area.left()..gauge_area.right() {
                buf[(x, end)].set_symbol(get_unicode_block(filled_height % 1.0));
            }
        }

        buf.set_span(label_col, label_row, label, clamped_label_width);
    }
}

fn get_unicode_block(frac: f64) -> &'static str {
    match (frac * 8.0).round() as u16 {
        1 => symbols::block::ONE_EIGHTH,
        2 => symbols::block::ONE_QUARTER,
        3 => symbols::block::THREE_EIGHTHS,
        4 => symbols::block::HALF,
        5 => symbols::block::FIVE_EIGHTHS,
        6 => symbols::block::THREE_QUARTERS,
        7 => symbols::block::SEVEN_EIGHTHS,
        8 => symbols::block::FULL,
        _ => " ",
    }
}
