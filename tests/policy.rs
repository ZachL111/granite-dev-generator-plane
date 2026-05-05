use granite_dev_generator_plane::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 91, capacity: 84, latency: 19, risk: 10, weight: 5 };
    assert_eq!(score(signal), 178);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 79, capacity: 90, latency: 16, risk: 12, weight: 9 };
    assert_eq!(score(signal), 162);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 80, capacity: 82, latency: 14, risk: 16, weight: 7 };
    assert_eq!(score(signal), 132);
    assert_eq!(classify(signal), "review");
}
