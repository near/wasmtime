import os
import csv
import sys
import argparse


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
    for line in sys.stdin:
        if "--> fail" in line or "--> pass" in line:
            _, _, test_path = line.partition(' ')
            test_name, _ = os.path.splitext(os.path.basename(test_path))
            status_map[test_name] = 'pass' if 'pass' in line else 'runtime error'


def write_csv(status_map):
    with open(state_csv_path, 'w', newline='') as csvfile:
        csvwriter = csv.writer(csvfile)
        csvwriter.writerow(['Test', 'Status'])
        status_list = sorted(status_map.items())
        csvwriter.writerows(status_list)
        csvwriter.writerow(['Total Passed', sum(1 for status in status_map.values() if status == 'pass')])
        csvwriter.writerow(['Amount of Tests', len(status_map)])


def assert_with_csv(status_map):
    with open(state_csv_path, newline='') as csvfile:
        csvreader = csv.reader(csvfile)
        csv_dict = {}
        for row in csvreader:
            if row[0] in ["Test", "Total Passed", "Amount of Tests"]:
                continue
            csv_dict[row[0]] = row[1]
        if csv_dict != status_map:
            diff = set(csv_dict.items()) ^ set(status_map.items())
            diff_keys = set(map(lambda x: x[0], diff))
            for key in diff_keys:
                print(f"Update for test {key}: {csv_dict[key]} => {status_map[key]}")
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
