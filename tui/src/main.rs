use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(lambda_calculator_tui::app)?;
    Ok(())
}
