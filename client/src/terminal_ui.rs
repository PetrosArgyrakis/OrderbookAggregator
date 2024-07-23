use std::io;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::{sync::broadcast::Receiver, task::JoinHandle};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    Terminal, widgets::{Block, Borders, Cell, Row, Table},
};

use crate::orderbook::Summary;

pub fn start(mut rx_summary: Receiver<Summary>) -> JoinHandle<()> {
    tokio::spawn(async move {
        enable_raw_mode();

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("");

        disable_raw_mode();

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );

        while let Ok(mut summary) = rx_summary.recv().await {
            terminal.draw(|f| draw(f, &mut summary));
        }
    })
}

fn draw<B: Backend>(f: &mut Frame<B>, summary: &mut Summary) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let header_cells = ["Id", "Exchange", "Price", "Amount"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

    let header = Row::new(header_cells).height(2).bottom_margin(1);

    let spread = summary.asks[0].price - summary.bids[0].price;

    let mut asks_row_cnt = 20;
    let asks = summary.asks.iter_mut().rev().map(|level| {
        let cells = vec![
            Cell::from(asks_row_cnt.to_string()),
            Cell::from(level.exchange.to_string()),
            Cell::from(level.price.to_string()),
            Cell::from(level.amount.to_string()),
        ];
        asks_row_cnt -= 1;
        Row::new(cells).height(1).bottom_margin(1)
    });

    let spread = vec![Row::new(vec![
        Cell::from("-"),
        Cell::from("-"),
        Cell::from("-"),
        Cell::from("-"),
    ])
        .height(1)
        .bottom_margin(1)];

    let mut bids_row_cnt: i32 = 1;
    let bids = summary.bids.iter_mut().map(|level| {
        let cells = vec![
            Cell::from(bids_row_cnt.to_string()),
            Cell::from(level.exchange.to_string()),
            Cell::from(level.price.to_string()),
            Cell::from(level.amount.to_string()),
        ];
        bids_row_cnt += 1;
        Row::new(cells).height(1).bottom_margin(1)
    });

    let t = Table::new(
        asks.into_iter()
            .chain(spread.into_iter())
            .chain(bids.into_iter()),
    )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Aggregated Orderbook")
                .title_alignment(tui::layout::Alignment::Center),
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .column_spacing(1);

    f.render_widget(t, rects[0]);
}
