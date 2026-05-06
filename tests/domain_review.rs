use granite_dev_generator_plane::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 59, slack: 33, drag: 9, confidence: 47 };
    assert_eq!(review_score(case), 171);
    assert_eq!(review_lane(case), "ship");
}
