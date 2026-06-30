use super::*;
use pretty_assertions::assert_eq;

#[test]
fn review_prompt_template_renders_base_branch_variant() {
    assert_eq!(
        render_review_prompt(&BASE_BRANCH_PROMPT_TEMPLATE, [("base_branch", "main")]),
        "Review the code changes against the base branch 'main'. First run `MERGE_BASE=$(git merge-base HEAD \"main\")` to compute a fresh merge base, then run `git diff \"$MERGE_BASE\"` to inspect the changes relative to main. Provide prioritized, actionable findings."
    );
}

#[test]
fn review_prompt_template_renders_commit_variant() {
    assert_eq!(
        review_prompt(
            &ReviewTarget::Commit {
                sha: "deadbeef".to_string(),
                title: None,
            },
            &AbsolutePathBuf::current_dir().expect("cwd"),
        )
        .expect("commit prompt should render"),
        "Review the code changes introduced by commit deadbeef. Provide prioritized, actionable findings."
    );
}

#[test]
fn review_prompt_template_renders_commit_variant_with_title() {
    assert_eq!(
        review_prompt(
            &ReviewTarget::Commit {
                sha: "deadbeef".to_string(),
                title: Some("Fix bug".to_string()),
            },
            &AbsolutePathBuf::current_dir().expect("cwd"),
        )
        .expect("commit prompt should render"),
        "Review the code changes introduced by commit deadbeef (\"Fix bug\"). Provide prioritized, actionable findings."
    );
}
