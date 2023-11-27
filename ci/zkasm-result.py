import os
import csv
import sys
import argparse
import json
import logging
from operator import countOf


CSV_FIELD_NAMES = ['Test', 'Status']
TEST_SUMMARY_FILE_PATH = "docs/zkasm/test_summary.csv"

parser = argparse.ArgumentParser(description='Example script to demonstrate flag usage.')
parser.add_argument('path', type=str, help='Path to a folder with tests')
parser.add_argument('--update', action='store_true', help='Flag to specify update')
args = parser.parse_args()
tests_path = args.path
generated_dir = f'{tests_path}/generated'
state_csv_path = f'{tests_path}/state.csv'
update = args.update


def check_compilation_status():
    status_map = {}
    for file in os.listdir(tests_path):
        if not file.endswith('.wat'):
            continue
        test_name = os.path.splitext(file)[0]
        zkasm_file = f'{test_name}.zkasm'
        status_map[test_name] = 'compilation success' if zkasm_file in os.listdir(generated_dir) else 'compilation failed'
    return status_map


def update_status_from_stdin(status_map):
    for test_result in json.load(sys.stdin):
        test_name, _ = os.path.splitext(os.path.basename(test_result["path"]))
        status_map[test_name] = test_result["status"]


def read_summary(filepath):
    with open(filepath, 'r', newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        return {row['Suite path']: row for row in reader}


def write_summary(filepath, summary):
    with open(filepath, 'w', newline='') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames = ['Suite path', 'Passing count', 'Total count'])
        writer.writeheader()
        for (path, value) in sorted(summary.items()):
            writer.writerow({'Suite path': path, **value})


def write_csv(status_map):
    with open(state_csv_path, 'w', newline='') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames = CSV_FIELD_NAMES)
        writer.writeheader()
        for (test_name, test_status) in sorted(status_map.items()):
            writer.writerow({'Test': test_name, 'Status': test_status})

    if os.path.exists(TEST_SUMMARY_FILE_PATH):
        summary = read_summary(TEST_SUMMARY_FILE_PATH)
    else:
        summary = {}

    summary[tests_path] = {
        'Passing count': countOf(status_map.values(), "pass"),
        'Total count': len(status_map),
    }
    write_summary(TEST_SUMMARY_FILE_PATH, summary)


def assert_with_csv(status_map):
    with open(state_csv_path, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        test_results = {row['Test']: row['Status'] for row in reader}
        if test_results != status_map:
            diff = set(test_results.items()) ^ set(status_map.items())
            diff_keys = set(map(lambda x: x[0], diff))
            for key in diff_keys:
                logging.info(f"Update for test {key}: {test_results[key]} => {status_map[key]}")
            return 1
    return 0


def main():
    status_map = check_compilation_status()
    update_status_from_stdin(status_map)
    if update:
        write_csv(status_map)
    else:
        if assert_with_csv(status_map) != 0:
            sys.exit(1)
    sys.exit(0)


if __name__ == "__main__":
    main()
