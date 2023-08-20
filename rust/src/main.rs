use gol::app::App;

fn main() -> anyhow::Result<()> {
    gol::app::setup_panic_hook();
    let (tick_rate, pattern) = gol::app::cli::parse();
    let app = App::new(pattern);
    gol::app::run(app, tick_rate)
}
