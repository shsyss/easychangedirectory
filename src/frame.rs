use std::path::{Path, PathBuf};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Row, Table, Widget},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, pwd: &Path, current_dir: &Path) {
    let size = f.size();
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);
    let lines = vec![
        Span::styled(pwd.to_string_lossy(), style),
        Span::styled(current_dir.to_string_lossy(), style),
    ];
    let filename = Row::new(lines);
    let file_list = vec![filename];

    let file_table = Table::new(file_list).block(
        Block::default()
            .title("file-manager")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(BorderType::Rounded),
    );

    f.render_widget(file_table, size);
}
