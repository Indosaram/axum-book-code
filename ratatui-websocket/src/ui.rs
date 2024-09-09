use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:3000/ws").await?;
    let (mut write, mut read) = ws_stream.split();

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Rock-Paper-Scissors").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        // 사용자 입력 처리 및 웹소켓 메시지 전송
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        write.send(Message::Text(input.trim().to_string())).await?;
    }
}