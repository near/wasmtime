import dataclasses
import os
import csv
import sys
import argparse
import json
from operator import countOf


CSV_FIELD_NAMES = ["Test", "Status", "Cycles"]
TEST_SUMMARY_FILE_PATH = "docs/zkasm/test_summary.csv"


@dataclasses.dataclass(frozen=True)
class TestResult:
    # Name of the test.
    test_name: str
    # Execution status of the test.
    status: str
    # Number of cycles it took to execute the test if it was successful.
    cycles: int | None

    def to_csv_record(self):
        return {
            "Test": self.test_name,
            "Status": self.status,
            "Cycles": self.cycles,
        }

    @staticmethod
    def from_csv_record(record):
        if record["Cycles"]:
            cycles = int(record["Cycles"])
        else:
            cycles = None

        return TestResult(
            test_name=record["Test"], status=record["Status"], cycles=cycles
        )


def record_failed_compilation_results(tests_path, generated_dir, test_results):
    for file in os.listdir(tests_path):
        if not file.endswith(".wat"):
            continue

        test_name = os.path.splitext(file)[0]
        zkasm_file = os.path.join(generated_dir, f"{test_name}.zkasm")
        if not os.path.exists(zkasm_file):
            test_results[test_name] = TestResult(
                test_name=test_name, status="compilation failed", cycles=None
            )


def parse_test_result(result_json):
    test_name, _ = os.path.splitext(os.path.basename(result_json["path"]))
    status = result_json["status"]
    if status == "pass":
        cycles = result_json["counters"]["cntSteps"]
    else:
        cycles = None

    return TestResult(test_name=test_name, status=status, cycles=cycles)


def read_test_execution_results(input_handle):
    test_results = {}
    for test_result_json in json.load(input_handle):
        test_result = parse_test_result(test_result_json)
        test_results[test_result.test_name] = test_result
    return test_results


def read_summary(filepath):
    with open(filepath, "r", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        return {row["Suite path"]: row for row in reader}


def write_summary(filepath, summary):
    with open(filepath, "w", newline="") as csvfile:
        writer = csv.DictWriter(
            csvfile,
            fieldnames=["Suite path", "Passing count", "Total count", "Total cycles"],
        )
        writer.writeheader()
        for path, value in sorted(summary.items()):
            writer.writerow({"Suite path": path, **value})


def read_test_results(test_results_path):
    with open(test_results_path, newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        return {row["Test"]: TestResult.from_csv_record(row) for row in reader}


def write_test_results(test_results, test_results_path, tests_path):
    with open(test_results_path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=CSV_FIELD_NAMES)
        writer.writeheader()
        for _, test_result in sorted(test_results.items()):
            writer.writerow(test_result.to_csv_record())

    if os.path.exists(TEST_SUMMARY_FILE_PATH):
        summary = read_summary(TEST_SUMMARY_FILE_PATH)
    else:
        summary = {}

    summary[tests_path] = {
        "Passing count": countOf((r.status for r in test_results.values()), "pass"),
        "Total count": len(test_results),
        "Total cycles": sum((r.cycles or 0 for r in test_results.values())),
    }
    write_summary(TEST_SUMMARY_FILE_PATH, summary)


def assert_dict_equals(actual, expected):
    if actual == expected:
        return

    diff = set(actual.items()) ^ set(expected.items())
    diff_keys = set(key for (key, _) in diff)
    diff_messages = [
        f"Update for test {key}: {expected[key]} => {actual[key]}" for key in diff_keys
    ]
    diff_message = "\n".join(diff_messages)
    raise AssertionError(
        f"Detected difference between the old and new state:\n{diff_message}"
    )


def main():
    parser = argparse.ArgumentParser(
        description="Example script to demonstrate flag usage."
    )
    parser.add_argument("path", type=str, help="Path to a folder with tests")
    parser.add_argument(
        "--update", action="store_true", help="Whether to update state CSVs"
    )
    args = parser.parse_args()
    tests_path = os.path.normpath(args.path)
    generated_dir = os.path.join(tests_path, "generated")
    test_results_path = os.path.join(tests_path, "state.csv")

    test_results = read_test_execution_results(sys.stdin)
    record_failed_compilation_results(tests_path, generated_dir, test_results)
    if args.update:
        write_test_results(test_results, test_results_path, tests_path)
    else:
        expected_test_results = read_test_results(test_results_path)
        assert_dict_equals(test_results, expected_test_results)


if __name__ == "__main__":
    main()
