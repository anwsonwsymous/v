use crossterm::event::KeyModifiers;
use tui::widgets::TableState;

pub enum ScrollMode {
    Normal = 1,
    Fast = 10,
    SuperFast = 50,
    LightSpeed = 200,
}

impl Into<ScrollMode> for KeyModifiers {
    fn into(self) -> ScrollMode {
        match self {
            _m if _m.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => ScrollMode::SuperFast,
            _m if _m.contains(KeyModifiers::SHIFT) => ScrollMode::Fast,
            _ => ScrollMode::Normal,
        }
    }
}

pub struct App {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    pub file_name: String,
    pub headers: bool,
}

impl App {
    pub fn new(items: Vec<Vec<String>>, file_name: String, headers: bool) -> App {
        App {
            state: TableState::default(),
            items,
            file_name,
            headers,
        }
    }

    pub fn next(&mut self, mode: ScrollMode) {
        let incr = mode as usize;
        let i = match self.state.selected() {
            Some(i) => {
                if i + incr > self.items.len() - 1 {
                    0
                } else {
                    i + incr
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, mode: ScrollMode) {
        let decr = mode as usize;
        let i = match self.state.selected() {
            Some(i) => {
                if (i as isize) - (decr as isize) < 0 {
                    self.items.len() - decr
                } else {
                    i - decr
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
