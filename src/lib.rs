pub fn tower02() {
    use tower_test::{assert_request_eq, mock};

    let (svc, mut handle) = mock::pair::<(), ()>();

    // Commenting out this code makes the one below work.
    //
    // Somehow rustc thinks these are the same even though they
    // are imported from different versions using the cargo rename feature.
    assert_request_eq!(handle, ());
}

pub async fn tower03() {
    use tower_test03::{assert_request_eq, mock};
    let (service, mut handle) = mock::pair::<(), ()>();

    assert_request_eq!(handle, ());
}
