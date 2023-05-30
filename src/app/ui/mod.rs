use chrono::{DateTime, Local, Utc};
use std::time::Duration;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Action, Hourglass, View};

struct Field {
    name: String,
    value: String,
}

pub fn build_ui<B: Backend>(f: &mut Frame<B>, app: &mut Hourglass) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    let task_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rects[0]);

    let header_cells = ["ID", "Description", "Age"].iter().map(|x| {
        Cell::from(*x).style(
            Style::default()
                .add_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::DIM),
        )
    });

    // for some reason, adding bottom_margin will mess up the underlines
    let header = Row::new(header_cells).style(Style::default()).height(1);
    // .bottom_margin(1);

    let rows = app.tasks.iter().map(|item| {
        let height = 1;

        let cells = vec![
            format!("{}", item.id),
            format!("{}", item.description),
            format_time(item.age.elapsed()),
        ]
        .into_iter()
        .map(|c| Cell::from(c));

        let mut style = Style::default();

        if item.completed {
            style = style
                .add_modifier(Modifier::CROSSED_OUT)
                .add_modifier(Modifier::DIM);
        }

        Row::new(cells).height(height).style(style)
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_symbol("*")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(10),
        ]);
    // .column_spacing(4);

    f.render_stateful_widget(table, task_layout[0], &mut app.table_state);

    // display details for task selected
    let details_block = Block::default().borders(Borders::ALL);

    if let Some(i) = app.table_state.selected() {
        let selected_task = app.tasks.get(i);

        if let Some(task) = selected_task {
            let time_format = "%b %d, %Y %I:%M %p";

            let task_description_block = render_task_detail(
                vec![String::from("Name"), String::from("Value")],
                vec![
                    Field {
                        name: String::from("ID"),
                        value: task.id.to_string(),
                    },
                    Field {
                        name: String::from("Description"),
                        value: task.description.clone(),
                    },
                    Field {
                        name: String::from("Age"),
                        value: format_time(task.age.elapsed()),
                    },
                    Field {
                        name: String::from("Created at"),
                        value: format!("{}", convert_utc_to_local(task.created_at, time_format)),
                    },
                    Field {
                        name: String::from("Modified at"),
                        value: format!("{}", convert_utc_to_local(task.modified_at, time_format)),
                    },
                ],
            )
            .block(details_block);
            f.render_widget(task_description_block, task_layout[1]);
        }
    }

    let mut title = String::from("Command");

    match &app.view {
        View::Task(action) => match action {
            Action::Add => title.push_str(" - Add task"),
            Action::Update => title.push_str(" - Update task"),
            _ => {}
        },
        View::Issues => {}
    }
    let command = Block::default().borders(Borders::ALL).title(title);

    f.render_widget(Paragraph::new(app.input.as_ref()).block(command), rects[1]);
}

fn format_time(time: Duration) -> String {
    let sec = time.as_secs();

    let year = 60 * 60 * 24 * 365;
    let month = 60 * 60 * 24 * 30;
    let week = 60 * 60 * 24 * 7;
    let day = 60 * 60 * 24;
    let hour = 60 * 60;
    let minute = 60;

    if sec >= 60 * 60 * 24 * 365 {
        return format!("{}y", sec / year);
    } else if sec >= 60 * 60 * 24 * 30 {
        return format!("{}y", sec / month);
    } else if sec >= 60 * 60 * 24 * 7 {
        return format!("{}w", sec / week);
    } else if sec >= 60 * 60 * 24 {
        return format!("{}d", sec / day);
    } else if sec >= 60 * 60 {
        return format!("{}h", sec / hour);
    } else if sec >= 60 {
        return format!("{}min", sec / minute);
    }

    format!("{}s", sec)
}

fn convert_utc_to_local(utc_time: DateTime<Utc>, time_format: &str) -> String {
    let local_time: DateTime<Local> = DateTime::from(utc_time);

    local_time.format(time_format).to_string()
}

fn render_task_detail<'a>(columns: Vec<String>, fields: Vec<Field>) -> Paragraph<'a> {
    let gap = 2;
    let column_width = 12;
    let border_char = "-";

    let mut spans: Vec<Spans> = vec![];

    let mut border_text: String = String::new();
    let mut header_text: String = String::new();

    // ======================= Column name ====================
    for col in columns.iter() {
        let header_text_gap = column_width + gap - col.len();

        header_text.push_str(
            format!(
                "{name}{yeet:<width$}",
                width = header_text_gap,
                name = col,
                yeet = ""
            )
            .as_str(),
        );

        border_text.push_str(
            format!(
                "{a}{b}",
                a = border_char.repeat(column_width),
                b = " ".repeat(gap)
            )
            .as_str(),
        );
    }

    spans.push(Spans::from(Span::styled(header_text, Style::default())));
    spans.push(Spans::from(Span::styled(border_text, Style::default())));

    // ====================== END COLUMN NAME ======================

    // ====================== COLUMN FIELDS ========================

    for field in fields.iter() {
        let field_text = format!(
            "{field}:{space}{value}",
            field = field.name,
            space = " ".repeat(column_width + gap - field.name.len() - 1),
            value = field.value
        );

        spans.push(Spans::from(Span::styled(
            field_text,
            Style::default().fg(Color::Red),
        )));
    }

    // ====================== END COLUMN FIELDS ========================

    Paragraph::new(spans.clone())
}
