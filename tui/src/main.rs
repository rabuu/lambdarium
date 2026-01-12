use anyhow::Result;

fn main() -> Result<()> {
    let mut app = lambda_calculator_tui::App::default();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
