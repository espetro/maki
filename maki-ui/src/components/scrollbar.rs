use std::sync::atomic::{AtomicBool, Ordering};

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

pub const SCROLLBAR_THUMB: &str = "\u{2590}";

static ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

/// Lets a caller skip counting rows it would only hand to a scrollbar nobody
/// draws. Worth asking when the count is not already lying around.
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn render_vertical_scrollbar(frame: &mut Frame, area: Rect, content_len: u32, position: u32) {
    if !is_enabled() {
        return;
    }
    let max_scroll = content_len.saturating_sub(u32::from(area.height));
    let mut state = ScrollbarState::default()
        .content_length(max_scroll as usize + 1)
        .position(position as usize);

    let scrollbar = scrollbar_widget();

    frame.render_stateful_widget(scrollbar, area, &mut state);
}

/// Paints the scrollbar into the border column immediately right of `inner`,
/// so no content cell is overwritten and copied text stays clean.
pub fn render_vertical_scrollbar_in_border(
    frame: &mut Frame,
    inner: Rect,
    content_len: u32,
    position: u32,
) {
    if inner.height <= 2 || inner.right() >= frame.area().width {
        return;
    }
    let rail = Rect::new(inner.right(), inner.y + 1, 1, inner.height - 2);
    render_vertical_scrollbar(frame, rail, content_len, position);
}

fn scrollbar_widget() -> Scrollbar<'static> {
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .thumb_symbol(SCROLLBAR_THUMB)
        // ListPicker renders highlighted rows over the scrollbar track; resetting
        // the thumb style keeps its color stable instead of inheriting row bg.
        .thumb_style(Style::new().fg(Color::Reset).bg(Color::Reset))
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None)
}
