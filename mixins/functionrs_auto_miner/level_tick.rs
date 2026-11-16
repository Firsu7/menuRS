fn functionrs_level_tick(game: &mut crate::game::Game<'_>) {
    // This method also runs on client ticks. Only run for a server level pass,
    // as the previous native_level_tick_pre injection did.
    if game.level_pass.running.is_some() {
        game.functionrs_level_tick();
    }
}
