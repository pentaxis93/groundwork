from pathlib import Path
import re
import unittest


ROOT = Path(__file__).resolve().parents[1]
SUBMIT_PROTOCOL_PATH = ROOT / "protocols" / "submit" / "PROTOCOL.md"
CHANGELOG_PATH = ROOT / "CHANGELOG.md"


def normalized_submit_protocol() -> str:
    return re.sub(r"\s+", " ", SUBMIT_PROTOCOL_PATH.read_text())


def normalized_changelog() -> str:
    return re.sub(r"\s+", " ", CHANGELOG_PATH.read_text())


def normalized_section(start: str, end: str) -> str:
    protocol = SUBMIT_PROTOCOL_PATH.read_text()
    section_start = protocol.index(start)
    section_end = protocol.index(end, section_start)
    return re.sub(r"\s+", " ", protocol[section_start:section_end])


class SubmitProtocolTests(unittest.TestCase):
    def test_existing_pr_with_local_work_is_a_push_path_not_a_stop_path(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("open PR exists and deliverable local work exists", protocol)
        self.assertIn("push to the existing PR branch", protocol)
        self.assertIn("pushed to existing PR", protocol)

    def test_open_pr_deliverability_uses_pr_head_regardless_of_upstream(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("Open PR exists", protocol)
        self.assertIn("regardless of whether the branch has upstream tracking", protocol)
        self.assertIn("classify local `HEAD`", protocol)
        self.assertIn("PR head SHA (`headRefOid`)", protocol)
        self.assertIn("git merge-base --is-ancestor", protocol)

    def test_no_open_pr_with_upstream_uses_remote_tracking_ref(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("No open PR and upstream exists", protocol)
        self.assertIn("branch's remote tracking ref", protocol)
        self.assertIn("git log @{upstream}..HEAD", protocol)

    def test_no_open_pr_with_upstream_already_pushed_still_opens_pr(self) -> None:
        step_4 = normalized_section("### 4. Resolve PR delivery path", "### 5. Push")
        step_5 = normalized_section("### 5. Push", "### 6. Create or identify PR")

        self.assertIn("No open PR and upstream exists", step_4)
        self.assertIn("run `git log main..HEAD`", step_4)
        self.assertIn("already-pushed branch still needs a PR", step_4)
        self.assertIn("Upstream-ahead commits decide whether a push is needed", step_4)
        self.assertIn("If no upstream-ahead commits exist", step_4)
        self.assertIn("open the new PR without a push", step_5)

    def test_no_open_pr_without_upstream_uses_first_push_semantics(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("No open PR and no upstream", protocol)
        self.assertIn("first-push semantics", protocol)

    def test_existing_pr_context_captures_pr_reference_and_head_sha(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("record the PR URL", protocol)
        self.assertIn("downstream PR reference", protocol)
        self.assertIn("PR head SHA", protocol)
        self.assertIn("headRefOid", protocol)

    def test_existing_pr_context_captures_pr_head_repo_and_ref(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("--json url,number,headRefOid,headRefName,headRepository,headRepositoryOwner", protocol)
        self.assertIn("PR head branch", protocol)
        self.assertIn("headRefName", protocol)
        self.assertIn("PR head repository", protocol)
        self.assertIn("headRepositoryOwner.login", protocol)
        self.assertIn("headRepository.name", protocol)

    def test_existing_pr_context_captures_pr_base_repo_from_same_gh_context(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("gh repo view --json nameWithOwner,url", protocol)
        self.assertIn("same repository context", protocol)
        self.assertIn("PR base repository", protocol)
        self.assertIn("base repository identity", protocol)

    def test_step_1_captures_pr_discovery_substrate(self) -> None:
        step_1 = normalized_section("### 1. Resolve context", "### 2. Ensure feature branch")

        self.assertIn("--json url,number,headRefOid,headRefName,headRepository,headRepositoryOwner", step_1)
        self.assertIn("gh repo view --json nameWithOwner,url", step_1)
        self.assertIn("record the PR URL", step_1)
        self.assertIn("PR number", step_1)
        self.assertIn("PR head SHA (`headRefOid`)", step_1)
        self.assertIn("PR head branch (`headRefName`)", step_1)
        self.assertIn("PR head repository", step_1)
        self.assertIn("headRepositoryOwner.login", step_1)
        self.assertIn("headRepository.name", step_1)
        self.assertIn("PR base repository", step_1)
        self.assertIn("git fetch <base-repo-remote> pull/<number>/head", step_1)
        self.assertIn("verify that `headRefOid` resolves", step_1)
        self.assertIn("post-commit deliverability classification", step_1)

    def test_existing_pr_discovery_fetches_resolvable_pr_head_before_classification(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("--json url,number,headRefOid,headRefName,headRepository,headRepositoryOwner", protocol)
        self.assertIn("matching the PR base repository", protocol)
        self.assertIn("git fetch <base-repo-remote> pull/<number>/head", protocol)
        self.assertIn("GitHub PR refspec", protocol)
        self.assertIn("verify that `headRefOid` resolves", protocol)
        self.assertLess(
            protocol.index("git fetch <base-repo-remote> pull/<number>/head"),
            protocol.index("git merge-base --is-ancestor"),
        )

    def test_pr_discovery_handles_zero_one_and_multiple_results(self) -> None:
        step_1 = normalized_section("### 1. Resolve context", "### 2. Ensure feature branch")

        self.assertIn("PR discovery handles result counts explicitly", step_1)
        self.assertIn("0 results", step_1)
        self.assertIn("no open PR", step_1)
        self.assertIn("1 result", step_1)
        self.assertIn("working PR", step_1)
        self.assertIn("2+ results", step_1)
        self.assertIn("disambiguate by head repository", step_1)

    def test_multiple_pr_discovery_uses_shared_remote_matching_for_head_repo(self) -> None:
        step_1 = normalized_section("### 1. Resolve context", "### 2. Ensure feature branch")

        self.assertIn("filter candidates by PR head repository", step_1)
        self.assertIn("local remotes", step_1)
        self.assertIn("shared GitHub remote matching", step_1)
        self.assertIn("effective push URL", step_1)
        self.assertIn("exactly one PR survives", step_1)

    def test_ambiguous_pr_discovery_stops_with_candidate_and_remote_context(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("Ambiguous PR discovery", protocol)
        self.assertIn("candidate PR URLs", protocol)
        self.assertIn("candidate head repositories", protocol)
        self.assertIn("operator's local remotes", protocol)
        self.assertIn("exactly one local remote", protocol)
        self.assertIn("exactly one candidate PR head repository", protocol)

    def test_existing_pr_base_remote_failure_stops_before_classification(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("No local remote matches the PR base repository", protocol)
        self.assertIn("Stop before PR head fetch and ancestry classification", protocol)
        self.assertIn("PR URL", protocol)
        self.assertIn("PR base repository", protocol)
        self.assertIn("matching local remote", protocol)

    def test_existing_pr_head_fetch_failure_stops_with_pr_url(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("PR head fetch or resolvability check fails", protocol)
        self.assertIn("recorded PR URL", protocol)
        self.assertIn("stop before ancestry classification", protocol)

    def test_existing_pr_classifies_all_ancestry_states(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("If `HEAD` and `headRefOid` are the same SHA", protocol)
        self.assertIn("If `HEAD` is an ancestor of `headRefOid`", protocol)
        self.assertIn("local checkout is behind the PR", protocol)
        self.assertIn("If `headRefOid` is an ancestor of `HEAD`", protocol)
        self.assertIn("deliverable through the existing PR update path", protocol)
        self.assertIn("If neither commit is an ancestor of the other", protocol)
        self.assertIn("Report the divergence and stop", protocol)

    def test_step_4_classifies_post_commit_deliverability_for_all_paths(self) -> None:
        step_4 = normalized_section("### 4. Resolve PR delivery path", "### 5. Push")

        self.assertIn("After commit analysis has completed", step_4)
        self.assertIn("classify current `HEAD`", step_4)
        self.assertIn("Open PR exists", step_4)
        self.assertIn("If `HEAD` and `headRefOid` are the same SHA", step_4)
        self.assertIn("If `HEAD` is an ancestor of `headRefOid`", step_4)
        self.assertIn("If `headRefOid` is an ancestor of `HEAD`", step_4)
        self.assertIn("If neither commit is an ancestor of the other", step_4)
        self.assertIn("No open PR and upstream exists", step_4)
        self.assertIn("No open PR and no upstream", step_4)

    def test_upstream_tracking_is_determined_at_step_4_classification_time(self) -> None:
        step_1 = normalized_section("### 1. Resolve context", "### 2. Ensure feature branch")
        step_4 = normalized_section("### 4. Resolve PR delivery path", "### 5. Push")

        self.assertNotIn("Whether upstream tracking exists", step_1)
        self.assertIn("determine whether upstream tracking exists", step_4)
        self.assertIn("classification-time branch state", step_4)
        self.assertIn("git rev-parse --abbrev-ref --symbolic-full-name @{upstream}", step_4)
        self.assertIn("A failing upstream lookup means no upstream exists", step_4)

    def test_existing_pr_push_targets_discovered_pr_head_ref(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("Existing PR update path", protocol)
        self.assertIn("local remote whose effective push URL points at the PR head repository", protocol)
        self.assertIn("git push <head-repo-remote> HEAD:<headRefName>", protocol)
        self.assertIn("Do not push the local branch name to `origin`", protocol)

    def test_existing_pr_remote_matching_is_part_of_github_forge_shape(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("GitHub-shaped commitment", protocol)
        self.assertIn("`gh` discovery", protocol)
        self.assertIn("`pull/<number>/head`", protocol)
        self.assertIn("shared GitHub remote matching", protocol)
        self.assertIn("git remote get-url --push <remote>", protocol)
        self.assertIn("git remote get-url <remote>", protocol)
        self.assertIn("SSH and HTTPS GitHub URLs", protocol)
        self.assertIn("strip `.git`", protocol)
        self.assertIn("lowercase `<owner>/<repo>`", protocol)
        self.assertIn("non-GitHub URL forms as non-matches", protocol)

    def test_existing_pr_without_matching_head_remote_stops_before_patch(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("No local remote matches the PR head repository", protocol)
        self.assertIn("Stop before push", protocol)
        self.assertIn("PR URL", protocol)
        self.assertIn("PR head repository", protocol)
        self.assertIn("headRefName", protocol)
        self.assertIn("Do not deliver a `patch` artifact", protocol)

    def test_new_pr_path_still_pushes_feature_branch_to_origin(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("New PR path", protocol)
        self.assertIn("push the feature branch to origin", protocol)
        self.assertIn("git push -u origin <branch>", protocol)
        self.assertIn("if upstream exists, run `git push`", protocol)

    def test_no_open_pr_without_upstream_checks_main_ahead_before_first_push(self) -> None:
        step_4 = normalized_section("### 4. Resolve PR delivery path", "### 5. Push")

        self.assertIn("No open PR and no upstream", step_4)
        self.assertIn("git log main..HEAD", step_4)
        self.assertIn("If commits exist", step_4)
        self.assertIn("first-push semantics", step_4)
        self.assertIn("If no commits exist", step_4)
        self.assertIn("clean-branch-no-changes", step_4)
        self.assertIn("stop", step_4)

    def test_analyze_and_commit_applies_to_both_pr_delivery_paths(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("shared step applies before either PR delivery path", protocol)
        self.assertIn("new PR or an existing PR update", protocol)

    def test_patch_artifact_is_complete_latest_pr_state_on_both_paths(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("complete latest PR state snapshot", protocol)
        self.assertIn(
            "same shape for both a newly opened PR and an updated existing PR",
            protocol,
        )

    def test_changelog_describes_base_fetch_and_first_push_emptiness(self) -> None:
        changelog = normalized_changelog()

        self.assertIn("matching PR base repository remote", changelog)
        self.assertIn("first-push", changelog)
        self.assertIn("git log main..HEAD", changelog)
        self.assertIn("already-pushed branch still needs a PR", changelog)
        self.assertIn("clean-branch-no-changes", changelog)

    def test_changelog_describes_classification_time_upstream_and_pr_disambiguation(self) -> None:
        changelog = normalized_changelog()

        self.assertIn("classification-time branch state", changelog)
        self.assertIn("upstream tracking", changelog)
        self.assertIn("multiple open PRs", changelog)
        self.assertIn("head repository", changelog)
        self.assertIn("local remote", changelog)


if __name__ == "__main__":
    unittest.main()
