//! A smoke test for windows compatibility

#[test]
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn test_double_register() {
    use mio::net::TcpListener;
    use mio::*;

    let poll = Poll::new().unwrap();

    // Create the listener
    let l = TcpListener::bind(&"127.0.0.1:0".parse().unwrap()).unwrap();

    // Register the listener with `Poll`
    poll.register(&l, Token(0), Interests::readable(), PollOpt::edge())
        .unwrap();
    assert!(poll
        .register(&l, Token(1), Interests::readable(), PollOpt::edge())
        .is_err());
}
