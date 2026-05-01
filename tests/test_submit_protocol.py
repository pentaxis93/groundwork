from pathlib import Path
import re
import unittest


ROOT = Path(__file__).resolve().parents[1]
SUBMIT_PROTOCOL_PATH = ROOT / "protocols" / "submit" / "PROTOCOL.md"


def normalized_submit_protocol() -> str:
    return re.sub(r"\s+", " ", SUBMIT_PROTOCOL_PATH.read_text())


class SubmitProtocolTests(unittest.TestCase):
    def test_existing_pr_with_local_work_is_a_push_path_not_a_stop_path(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("open PR exists and deliverable local work exists", protocol)
        self.assertIn("push to the existing PR branch", protocol)
        self.assertIn("pushed to existing PR", protocol)

    def test_deliverable_commits_are_measured_against_remote_tracking_ref(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("branch's remote tracking ref", protocol)
        self.assertNotIn("git log origin/main..HEAD", protocol)
        self.assertNotIn("git log main..HEAD", protocol)

    def test_deliverability_distinguishes_no_upstream_existing_pr_by_pr_head(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("No upstream and no open PR", protocol)
        self.assertIn("No upstream and open PR", protocol)
        self.assertIn("PR head SHA", protocol)
        self.assertIn("headRefOid", protocol)
        self.assertIn("git rev-parse HEAD", protocol)

    def test_existing_pr_context_captures_pr_reference_and_head_sha(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("record the PR URL", protocol)
        self.assertIn("downstream PR reference", protocol)
        self.assertIn("PR head SHA", protocol)
        self.assertIn("headRefOid", protocol)

    def test_existing_pr_discovery_fetches_resolvable_pr_head_before_classification(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("--json url,number,headRefOid", protocol)
        self.assertIn("git fetch origin pull/<number>/head", protocol)
        self.assertIn("GitHub PR refspec", protocol)
        self.assertIn("verify that `headRefOid` resolves", protocol)
        self.assertLess(
            protocol.index("git fetch origin pull/<number>/head"),
            protocol.index("git merge-base --is-ancestor"),
        )

    def test_existing_pr_head_fetch_failure_stops_with_pr_url(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("PR head fetch or resolvability check fails", protocol)
        self.assertIn("recorded PR URL", protocol)
        self.assertIn("stop before ancestry classification", protocol)

    def test_no_upstream_existing_pr_classifies_all_ancestry_states(self) -> None:
        protocol = normalized_submit_protocol()

        self.assertIn("If `HEAD` and `headRefOid` are the same SHA", protocol)
        self.assertIn("If `HEAD` is an ancestor of `headRefOid`", protocol)
        self.assertIn("local checkout is behind the PR", protocol)
        self.assertIn("If `headRefOid` is an ancestor of `HEAD`", protocol)
        self.assertIn("deliverable through the existing PR update path", protocol)
        self.assertIn("If neither commit is an ancestor of the other", protocol)
        self.assertIn("Report the divergence and stop", protocol)

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


if __name__ == "__main__":
    unittest.main()
