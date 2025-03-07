use google_course::{
    borrow, btree_run, drop_release, elevator_run, generics_run, lifetime, math_agi_run_proxy,
    math_run, player_run, std_api_run, trait_run,
};
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
    generics_run();
    std_api_run();
    drop_release();

    btree_run();
    borrow::borrow_run();
    lifetime::lifetime_run();
}
