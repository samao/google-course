use google_course::{
    borrow, btree_run, drop_release, elevator_run, error_api::error_api, generics_run, gui_run,
    iterator::iterator_run, lifetime, math_agi_run_proxy, math_run, player_run, proto, std_api_run,
    trait_run,
};
use tracing::{Level, info, instrument};
use tracing_subscriber::FmtSubscriber;

#[instrument(name = "main")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
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
    proto::proto_run();

    iterator_run();
    gui_run();
    error_api();
    Ok(())
}
