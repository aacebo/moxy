#[test]
fn matches() {
    enumerate::apply! {
        enum Status {
            Pending,
            Running,
            Complete,
            Failed,
        }
    };

    assert_eq!(Status::Pending.as_str(), "Pending");
    assert_eq!(Status::Running.as_str(), "Running");
    assert_eq!(Status::Complete.as_str(), "Complete");
    assert_eq!(Status::Failed.as_str(), "Failed");
}
