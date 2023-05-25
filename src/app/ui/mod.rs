use std::time::Duration;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Action, Hourglass, View};

pub fn build_ui<'a, B: Backend>(f: &mut Frame<B>, app: &mut Hourglass) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(35),
            Constraint::Percentage(5),
        ])
        .split(f.size());

    let header_cells = ["ID", "Description", "Age"]
        .iter()
        .map(|x| Cell::from(*x).style(Style::default()));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

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

        Row::new(cells).height(height).bottom_margin(1).style(style)
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_symbol("*")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(70),
            Constraint::Percentage(10),
        ]);

    f.render_stateful_widget(table, rects[0], &mut app.table_state);

    // display details for task selected
    let details_block = Block::default().borders(Borders::ALL);

    if let Some(i) = app.table_state.selected() {
        let selected_task = app.tasks.get(i);

        if let Some(task) = selected_task {
            let task_description_block = Paragraph::new(vec![
                Spans::from(Span::styled("Name          Value", Style::default())),
                Spans::from(Span::styled(
                    "------------  ------------------",
                    Style::default(),
                )),
                Spans::from(Span::styled(
                    format!("ID:           {}", task.id),
                    Style::default().fg(Color::Red),
                )),
                Spans::from(Span::styled(
                    format!("Description:  {}", task.description),
                    Style::default().fg(Color::Red),
                )),
                // TODO: show age of task
            ])
            .block(details_block);

            f.render_widget(task_description_block, rects[1]);
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

    f.render_widget(Paragraph::new(app.input.as_ref()).block(command), rects[2]);
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
