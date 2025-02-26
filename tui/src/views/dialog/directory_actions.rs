use {
    crate::context::{notebook::DIRECTORY_ACTIONS, NotebookContext},
    ratatui::{
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Color, Style},
        widgets::{Block, Clear, HighlightSpacing, List, ListDirection, Padding},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, context: &mut NotebookContext) {
    let [area] = Layout::horizontal([Length(28)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(9)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title("Directory Actions")
        .title_alignment(Alignment::Center);
    let list = List::new(DIRECTORY_ACTIONS)
        .block(block)
        .highlight_style(Style::new().fg(Color::White).bg(Color::Blue))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut context.directory_actions_state);
}
