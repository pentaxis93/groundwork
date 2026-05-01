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
