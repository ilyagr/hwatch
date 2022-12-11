// Copyright (c) 2022 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Cell, Row, Table, TableState},
    Frame,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct History {
    pub timestamp: String,
    pub status: bool,
    pub num: u16,
}

pub struct HistoryArea {
    ///
    pub area: tui::layout::Rect,

    pub active: bool,

    ///
    data: Vec<Vec<History>>,

    ///
    state: TableState,
}

/// History Area Object Trait
impl HistoryArea {
    pub fn new() -> Self {
        //! new Self
        Self {
            area: tui::layout::Rect::new(0, 0, 0, 0),
            active: false,
            data: vec![vec![History {
                timestamp: "latest                 ".to_string(),
                status: true,
                num: 0,
            }]],
            state: TableState::default(),
        }
    }

    pub fn set_area(&mut self, area: tui::layout::Rect) {
        self.area = area;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_latest_status(&mut self, latest_status: bool) {
        self.data[0][0].status = latest_status;
    }

    pub fn update(&mut self, timestamp: String, status: bool, num: u16) {
        self.set_latest_status(status);

        // insert latest timestamp
        self.data.insert(
            1,
            vec![History {
                timestamp,
                status,
                num,
            }],
        );
    }

    ///
    pub fn reset_history_data(&mut self, data: Vec<Vec<History>>) {
        // update data
        self.data = data;

        // set select num
        self.state.select(Some(0));
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        // insert latest timestamp
        const LATEST_COLOR: Color = Color::Blue;
        let draw_data = &self.data;

        let rows = draw_data.iter().enumerate().map(|(ix, item)| {
            // set table height
            let height = item
                .iter()
                .map(|content| content.timestamp.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            // set cell data
            let cells = item.iter().map(|c| {
                let cell_style = Style::default().fg(match ix {
                    0 => LATEST_COLOR,
                    _ => match c.status {
                        true => Color::Green,
                        false => Color::Red,
                    },
                });
                Cell::from(Span::styled(c.timestamp.as_str(), cell_style))
            });

            Row::new(cells).height(height as u16)
        });

        let base_selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let selected_style = match self.active {
            true => match self.get_state_select() == 0 {
                true => base_selected_style.fg(LATEST_COLOR), // Necessary to make >> blue
                false => base_selected_style,
            },
            false => base_selected_style.fg(Color::DarkGray),
        };
        let table = Table::new(rows)
            .block(Block::default())
            .highlight_style(selected_style)
            .highlight_symbol(">>")
            .widths(&[Constraint::Percentage(100)]);

        frame.render_stateful_widget(table, self.area, &mut self.state);
    }

    pub fn get_state_select(&self) -> usize {
        let i = match self.state.selected() {
            Some(i) => i,
            None => self.data.len() - 1,
        };

        self.data[i][0].num as usize
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.data.len() - 1 {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // NOTE: TODO:
    // It will not be supported until the following issues are resolved.
    //     - https://github.com/fdehau/tui-rs/issues/495
    //
    // pub fn click_row(&mut self, row: u16) {
    //     let select_num = row as usize;
    //     self.state.select(Some(select_num));
    // }
}
