use conventional_commit_parser::parse_summary;

mod assertions;
use assertions::*;
use conventional_commit_parser::commit::CommitType;

#[test]
fn parse_scope_with_whitespace() {
    let summary = "feat(some scope): message";
    let result = parse_summary(summary);

    assert_scope(&result, "some scope");
    assert_commit_type(&result, CommitType::Feature);
    assert_summary(&result, "message");
    assert_no_footers(&result);
    assert_no_body(&result);
}
