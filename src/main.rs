use clap::Parser;

mod app;

fn main() -> Result<(), ()> {
    let cli = app::CliArgs::parse();
    app::run(cli)
}
