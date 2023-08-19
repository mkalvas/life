use gol::app::App;

fn main() -> anyhow::Result<()> {
    gol::app::setup_panic_hook();
    // let cli = gol::cli::parse();
    let app = App::new();
    gol::app::run(app)
}
