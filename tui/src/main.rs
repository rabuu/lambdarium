use anyhow::Result;

fn main() -> Result<()> {
    lambdarium_tui::run_app()?;
    Ok(())
}
