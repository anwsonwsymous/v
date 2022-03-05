use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Layout, Direction, Constraint, Rect, Alignment};
use tui::widgets::{Cell, Block, Borders, Paragraph, Row, Table};
use tui::style::{Style, Color, Modifier};
use tui::text::{Spans, Span};
use crate::app::*;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(f.size().height as u32 - 2, f.size().height as u32),
            Constraint::Ratio(1, f.size().height as u32),
        ].as_ref())
        .split(f.size());

    draw_table(f, app, &layout);
    draw_info(f, app, &layout);
}

fn draw_info<B: Backend>(f: &mut Frame<B>, app: &mut App, layout: &Vec<Rect>) {
    let b = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::Yellow));

    let s = Style::default().fg(Color::White).add_modifier(Modifier::ITALIC);

    let mut spans_vec = vec![
        Span::styled(format!("Lines: {}", app.items.len()), s)
    ];
    if let Some(selected) = app.state.selected() {
        spans_vec.push(Span::styled(format!("    Selected: {}", (selected) + 1), s));
    }

    let p = Paragraph::new(Spans::from(spans_vec))
        .alignment(Alignment::Left)
        .block(b);
    f.render_widget(p, layout[1]);
}

fn draw_table<B: Backend>(f: &mut Frame<B>, app: &mut App, layout: &Vec<Rect>) {
    let selected_item_style = Style::default()
        .fg(Color::Red)
        .bg(Color::White)
        .add_modifier(Modifier::REVERSED);

    let table_style = Style::default()
        .bg(Color::Black)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);

    let rows: Vec<Row> = if app.items.len() > 1000 {
        // Optimization for big files (SHOW MAX 200 rows)
        let selected_index = app.state.selected().unwrap_or(0);

        let top_index = if selected_index + 100 > app.items.len() {
            app.items.len() - 1
        } else {
            selected_index + 100
        };
        let bot_index = if (selected_index as isize) - 100 < 0 {
            0
        } else {
            selected_index - 100
        };

        app.items.iter().enumerate()
            .map(|(i, cells)| {
                if i <= top_index && i >= bot_index {
                    return Row::new(cells.iter().map(|cell| Cell::from(cell.to_owned())));
                }
                Row::default()
            }).collect()
    } else {
        app.items.iter().map(|cells| {
            Row::new(cells.iter().map(|cell| Cell::from(cell.to_owned())))
        }).collect()
    };

    let cells_count = app.items[0].len();
    let constraints = vec![Constraint::Ratio(1, cells_count as u32); cells_count];

    let title = Span::styled(
    format!("Reading file: {}", app.file_name),
        Style::default().fg(Color::Green).add_modifier(Modifier::UNDERLINED)
    );

    let t = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(title)
                .title_alignment(Alignment::Center)
        )
        .column_spacing(0)
        .style(table_style)
        .highlight_style(selected_item_style)
        .widths(&constraints);

    f.render_stateful_widget(t, layout[0], &mut app.state);
}
