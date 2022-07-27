use std::{
    path::{Path, PathBuf},
    vec,
};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Row, Table, Widget},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, pwd: &Path, current_dir: &Path) {
    let size = f.size();
    let lines = vec![
        Span::raw(pwd.to_string_lossy()),
        Span::raw(current_dir.to_string_lossy()),
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
