use node::clock::ChainEpochClock;

#[test]
fn create_chain_epoch_clock() {
    let utc_timestamp = 1_574_286_946_904;
    let clock = ChainEpochClock::new(utc_timestamp);
    assert_eq!(clock.get_time().timestamp(), utc_timestamp);
}
