use google_course::{elevator_run, math_agi_run_proxy, math_run, player_run, trait_run};
use tracing::{Level, info, instrument};
use tracing_subscriber::FmtSubscriber;

#[instrument(name = "main")]
fn main() {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("This is the main function");
    player_run();
    elevator_run();
    math_run();
    math_agi_run_proxy();

    trait_run();
}
