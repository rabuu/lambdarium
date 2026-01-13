use anyhow::Result;

fn main() -> Result<()> {
    lambda_calculator_tui::run_app()?;
    Ok(())
}
