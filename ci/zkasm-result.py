import os
import csv
import sys
import argparse
import json
from operator import countOf


CSV_FIELD_NAMES = ["Test", "Status"]
TEST_SUMMARY_FILE_PATH = "docs/zkasm/test_summary.csv"


def check_compilation_status(tests_path, generated_dir):
    test_results = {}
    for file in os.listdir(tests_path):
        if not file.endswith(".wat"):
            continue

        test_name = os.path.splitext(file)[0]
        zkasm_file = os.path.join(generated_dir, f"{test_name}.zkasm")
        test_results[test_name] = (
            "compilation success"
            if os.path.exists(zkasm_file)
            else "compilation failed"
        )
    return test_results


def update_test_results_from_stdin(test_results):
    for test_result in json.load(sys.stdin):
        test_name, _ = os.path.splitext(os.path.basename(test_result["path"]))
        test_results[test_name] = test_result["status"]


def read_summary(filepath):
    with open(filepath, "r", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        return {row["Suite path"]: row for row in reader}


def write_summary(filepath, summary):
    with open(filepath, "w", newline="") as csvfile:
        writer = csv.DictWriter(
            csvfile, fieldnames=["Suite path", "Passing count", "Total count"]
        )
        writer.writeheader()
        for path, value in sorted(summary.items()):
            writer.writerow({"Suite path": path, **value})


def read_test_results(test_results_path):
    with open(test_results_path, newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        return {row["Test"]: row["Status"] for row in reader}


def write_test_results(test_results, test_results_path, tests_path):
    with open(test_results_path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=CSV_FIELD_NAMES)
        writer.writeheader()
        for test_name, test_status in sorted(test_results.items()):
            writer.writerow({"Test": test_name, "Status": test_status})

    if os.path.exists(TEST_SUMMARY_FILE_PATH):
        summary = read_summary(TEST_SUMMARY_FILE_PATH)
    else:
        summary = {}

    summary[tests_path] = {
        "Passing count": countOf(test_results.values(), "pass"),
        "Total count": len(test_results),
    }
    write_summary(TEST_SUMMARY_FILE_PATH, summary)


def assert_dict_equals(actual, expected):
    if actual == expected:
        return

    diff = set(actual.items()) ^ set(expected.items())
    diff_keys = set(key for (key, _) in diff)
    diff_messages = [
        f"Update for test {key}: {expected[key]} => {actual[key]}\n"
        for key in diff_keys
    ]
    raise AssertionError(
        f"Detected difference between the old and new state: {diff_messages}"
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
    tests_path = args.path
    generated_dir = f"{tests_path}/generated"
    test_results_path = f"{tests_path}/state.csv"

    test_results = check_compilation_status(tests_path, generated_dir)
    update_test_results_from_stdin(test_results)
    if args.update:
        write_test_results(test_results, test_results_path, tests_path)
    else:
        expected_test_results = read_test_results(test_results_path)
        assert_dict_equals(test_results, expected_test_results)


if __name__ == "__main__":
    main()
