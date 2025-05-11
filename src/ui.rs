use crate::element::Element;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};

pub struct PeriodicTableUi {
    pub table_layout: Vec<Vec<Option<Element>>>,
    pub selected_element: Option<Element>,
    pub cursor_position: (usize, usize),
}

impl PeriodicTableUi {
    pub fn new(table_layout: Vec<Vec<Option<Element>>>) -> Self {
        // Initialize with first valid element selected
        let mut ui = Self {
            table_layout,
            selected_element: None,
            cursor_position: (0, 0),
        };

        // Find the first valid element position
        ui.find_and_select_valid_element();
        ui
    }

    // Helper function to find a valid element position
    fn find_and_select_valid_element(&mut self) {
        for (row_idx, row) in self.table_layout.iter().enumerate() {
            for (col_idx, element_opt) in row.iter().enumerate() {
                if element_opt.is_some() {
                    self.cursor_position = (row_idx, col_idx);
                    self.update_selected_element();
                    return;
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Title
                Constraint::Min(10),    // Table
                Constraint::Length(10), // Element details
            ])
            .split(size);

        // Render title with helpful instructions
        frame.render_widget(
            Paragraph::new("Periodic Table (Use arrow keys to navigate, Enter to select, q to quit)")
                .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            chunks[0],
        );

        // Render periodic table
        self.render_table(frame, chunks[1]);

        // Render element details
        self.render_element_details(frame, chunks[2]);
    }

    fn render_table(&self, frame: &mut Frame, area: Rect) {
        // Calculate cell size based on available area
        let rows = self.table_layout.len();
        let cols = if rows > 0 { self.table_layout[0].len() } else { 0 };
        
        let cell_width = 5;
        let cell_height = 3;
        
        let total_width = cols * cell_width;
        let total_height = rows * cell_height;
        
        // Center the table in the available area
        let x_offset = (area.width.saturating_sub(total_width as u16)) / 2;
        let y_offset = (area.height.saturating_sub(total_height as u16)) / 2;
        
        // Render each element
        for (row_idx, row) in self.table_layout.iter().enumerate() {
            for (col_idx, element_opt) in row.iter().enumerate() {
                if let Some(element) = element_opt {
                    let cell_x = x_offset + (col_idx * cell_width) as u16;
                    let cell_y = y_offset + (row_idx * cell_height) as u16;
                    
                    if cell_x + cell_width as u16 <= area.width && cell_y + cell_height as u16 <= area.height {
                        let cell_rect = Rect::new(
                            area.x + cell_x,
                            area.y + cell_y,
                            cell_width as u16,
                            cell_height as u16,
                        );
                        
                        // Convert RGB color to terminal color
                        let (r, g, b) = element.color();
                        let bg_color = Color::Rgb(r, g, b);
                        
                        // Highlight selected element
                        let is_selected = self.cursor_position == (row_idx, col_idx);
                        let (border_style, text_style) = if is_selected {
                            (
                                Style::default().fg(Color::White).bg(bg_color)
                                    .add_modifier(Modifier::BOLD),
                                Style::default().fg(Color::White).bg(bg_color)
                                    .add_modifier(Modifier::BOLD)
                            )
                        } else {
                            (
                                Style::default().fg(Color::White).bg(bg_color),
                                Style::default().fg(Color::Black).bg(bg_color)
                            )
                        };
                        
                        // Element content
                        let symbol = Line::from(vec![
                            Span::styled(
                                element.symbol.clone(),
                                text_style
                            )
                        ]);

                        
                        // Create element widget
                        let element_widget = Paragraph::new(vec![
                            symbol,
                        ])
                        .block(Block::default()
                            .borders(Borders::ALL)
                            .border_style(border_style))
                        .alignment(ratatui::layout::Alignment::Center);
                        
                        frame.render_widget(element_widget, cell_rect);
                    }
                }
            }
        }
    }

    fn render_element_details(&self, frame: &mut Frame, area: Rect) {
        let detail_block = Block::default()
            .title(Span::styled("Element Details", Style::default().add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        
        let inner_area = detail_block.inner(area);
        frame.render_widget(detail_block, area);
        
        if let Some(element) = &self.selected_element {
            let details = vec![
                Line::from(vec![
                    Span::styled(
                        format!("{}: {} ({})", element.number, element.name, element.symbol),
                        Style::default().add_modifier(Modifier::BOLD),
                    )
                ]),
                Line::from(vec![
                    Span::raw(format!("Atomic Mass: {}", element.mass))
                ]),
                Line::from(vec![
                    Span::raw(format!("Category: {:?}", element.category))
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw(&element.description)
                ]),
            ];
            
            let paragraph = Paragraph::new(details)
                .wrap(Wrap { trim: true })
                .style(Style::default());
            
            frame.render_widget(paragraph, inner_area);
        } else {
            let text = vec![
                Line::from(vec![
                    Span::raw("Select an element to see details")
                ]),
            ];
            
            let paragraph = Paragraph::new(text)
                .style(Style::default());
            
            frame.render_widget(paragraph, inner_area);
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                return false;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if self.cursor_position.0 > 0 {
                    let old_row = self.cursor_position.0;
                    self.cursor_position.0 -= 1;

                    // If we moved to an empty spot, try to find a valid element in this row
                    if !self.is_valid_position() {
                        self.find_valid_element_in_row(self.cursor_position.0);
                        // If still not valid, go back to original position
                        if !self.is_valid_position() {
                            self.cursor_position.0 = old_row;
                        }
                    }
                    self.update_selected_element();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if self.cursor_position.0 < self.table_layout.len() - 1 {
                    let old_row = self.cursor_position.0;
                    self.cursor_position.0 += 1;

                    // If we moved to an empty spot, try to find a valid element in this row
                    if !self.is_valid_position() {
                        self.find_valid_element_in_row(self.cursor_position.0);
                        // If still not valid, go back to original position
                        if !self.is_valid_position() {
                            self.cursor_position.0 = old_row;
                        }
                    }
                    self.update_selected_element();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if self.cursor_position.1 > 0 {
                    let old_col = self.cursor_position.1;
                    self.cursor_position.1 -= 1;

                    // Keep moving left until we find a valid element or reach the edge
                    while !self.is_valid_position() && self.cursor_position.1 > 0 {
                        self.cursor_position.1 -= 1;
                    }

                    // If we still don't have a valid element, go back to original position
                    if !self.is_valid_position() {
                        self.cursor_position.1 = old_col;
                    }

                    self.update_selected_element();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if !self.table_layout.is_empty() && self.cursor_position.1 < self.table_layout[0].len() - 1 {
                    let old_col = self.cursor_position.1;
                    self.cursor_position.1 += 1;

                    // Keep moving right until we find a valid element or reach the edge
                    while !self.is_valid_position() &&
                          self.cursor_position.1 < self.table_layout[0].len() - 1 {
                        self.cursor_position.1 += 1;
                    }

                    // If we still don't have a valid element, go back to original position
                    if !self.is_valid_position() {
                        self.cursor_position.1 = old_col;
                    }

                    self.update_selected_element();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.update_selected_element();
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) => {
                // Convert mouse coordinates to table coordinates
                // This is simplified and would need adjustment based on actual rendering
                let cell_width = 5;
                let cell_height = 3;
                
                // Calculate table offset
                let rows = self.table_layout.len();
                let cols = if rows > 0 { self.table_layout[0].len() } else { 0 };
                
                let total_width = cols * cell_width;
                let total_height = rows * cell_height;
                
                // Match the logic in render_table for consistent calculations
                let x_offset = (*column as u16).saturating_sub((total_width as u16) / 2);
                let y_offset = (*row as u16).saturating_sub((total_height as u16) / 2);

                let table_col = (x_offset as usize) / cell_width;
                let table_row = (y_offset as usize) / cell_height;

                if table_row < self.table_layout.len() &&
                   table_col < self.table_layout[0].len() &&
                   self.table_layout[table_row][table_col].is_some() {
                    self.cursor_position = (table_row, table_col);
                    self.update_selected_element();
                }
            }
            _ => {}
        }
        true
    }

    // Check if current position has a valid element
    fn is_valid_position(&self) -> bool {
        let (row, col) = self.cursor_position;
        if row < self.table_layout.len() && col < self.table_layout[row].len() {
            self.table_layout[row][col].is_some()
        } else {
            false
        }
    }

    // Find a valid element in the specified row
    fn find_valid_element_in_row(&mut self, row: usize) {
        if row >= self.table_layout.len() {
            return;
        }

        for col in 0..self.table_layout[row].len() {
            if self.table_layout[row][col].is_some() {
                self.cursor_position.1 = col;
                return;
            }
        }
    }

    fn update_selected_element(&mut self) {
        let (row, col) = self.cursor_position;
        if row < self.table_layout.len() && col < self.table_layout[row].len() {
            self.selected_element = self.table_layout[row][col].clone();
        }
    }
}