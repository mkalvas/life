use gol::app::state::App;

fn main() -> anyhow::Result<()> {
    gol::app::tui::setup_panic_hook();
    // let cli = gol::cli::parse();
    let app = App::new();
    gol::app::tui::run(app)
}
