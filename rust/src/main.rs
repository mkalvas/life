use gol::app::App;

fn main() -> anyhow::Result<()> {
    gol::app::setup_panic_hook();
    let (pattern, tui_opts) = gol::app::cli::parse();
    let app = App::new(pattern);
    gol::app::run(app, tui_opts)
}
