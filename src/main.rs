mod element;
mod elements_data;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use ui::PeriodicTableUi;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let table_layout = elements_data::get_periodic_table_layout();
    let mut app = PeriodicTableUi::new(table_layout);

    // Run the main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut PeriodicTableUi,
) -> io::Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| app.render(f))?;

        // Poll for events with a 100ms timeout
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Ok(event) = event::read() {
                // If handle_event returns false (quit requested), exit the loop
                if !app.handle_event(&event) {
                    return Ok(());
                }

                // Redraw immediately after an input event for better responsiveness
                terminal.draw(|f| app.render(f))?;
            }
        }
    }
}
