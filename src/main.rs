use std::{error::Error, process};

use grep_tui::{restore_terminal, run_app, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal().unwrap_or_else(|err| {
        eprintln!("Problem while setting up terminal: {err}");
        process::exit(1);
    });

    let res = run_app(&mut terminal);

    if let Err(err) = restore_terminal(terminal) {
        eprintln!("Failed to restore the terminal: {err}");
    }

    if let Err(err) = res {
        eprintln!("Application error: {:?}", err)
    }

    Ok(())
}
