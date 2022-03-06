use crate::app::*;
use crate::ui::*;
use crossterm::event::{
    read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn run(rows: Vec<Vec<String>>, file_name: String, headers: bool) {
    // setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // UI
    let app = App::new(rows, file_name, headers);
    let res = run_loop(&mut terminal, app);

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }
}

fn run_loop<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(f, &mut app))?;

        match read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(key.modifiers.into()),
                KeyCode::Up => app.previous(key.modifiers.into()),
                KeyCode::Left | KeyCode::Esc => app.unselect(),
                KeyCode::PageUp => app.previous(ScrollMode::LightSpeed),
                KeyCode::PageDown => app.next(ScrollMode::LightSpeed),
                _ => {}
            },
            Event::Mouse(event) => match event.kind {
                MouseEventKind::ScrollUp => app.previous(event.modifiers.into()),
                MouseEventKind::ScrollDown => app.next(event.modifiers.into()),
                _ => {}
            },
            _ => {}
        }
    }
}
