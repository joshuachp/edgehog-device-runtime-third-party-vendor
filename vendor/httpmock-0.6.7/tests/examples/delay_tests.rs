use httpmock::prelude::*;
use isahc::get;
use std::time::{Duration, SystemTime};

#[test]
fn delay_test() {
    // Arrange
    let start_time = SystemTime::now();
    let delay = Duration::from_secs(3);

    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.path("/delay");
        then.status(200).delay(delay);
    });

    // Act: Send the HTTP request
    let response = get(server.url("/delay")).unwrap();

    // Assert
    mock.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(start_time.elapsed().unwrap() > delay, true);
}
