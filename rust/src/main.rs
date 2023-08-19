use gol::app::{App, InitPattern};

fn main() -> anyhow::Result<()> {
    gol::app::setup_panic_hook();
    // let cli = gol::cli::parse();
    let app = App::new(InitPattern::Random);
    gol::app::run(app)
}
