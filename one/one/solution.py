from itertools import combinations
from functools import reduce
from operator import mul
from pathlib import Path

import click


class NotFound(Exception):
    pass


def find_candidate(*, input_records, target_sum, num_entries):
    """Finds the combination of size `num_entries` from the `input_records` that will sum to the `target_sum`

    Args:
        input_records (int[]): The records to search
        target_sum (int): The target sum
        num_entries (int): The number of entries that should be searched as the result. For example, for a num_entries of size 2, the return value is (X, Y)

    Raises:
        NotFound: A matching set of entries could not be found that sum to the `target_sum`

    Returns:
        tuple: A tuple of size `num_entries` that add to the `target_sum`
    """
    check_set = set(input_records)
    for candidate_set in combinations(input_records, num_entries - 1):
        possibility = target_sum - sum(candidate_set)
        if possibility in check_set and possibility not in candidate_set:
            return (*candidate_set, possibility)

    raise NotFound


@click.command()
@click.argument("input_file", type=click.File("r"))
@click.option("-n", "--num-entries", type=int, default=2)
@click.option("-t", "--target", type=int, default=2020)
def run(input_file, num_entries, target):
    records = [int(line) for line in input_file if line.strip()]

    print(
        reduce(
            mul,
            find_candidate(
                input_records=records,
                target_sum=target,
                num_entries=num_entries,
            ),
        )
    )
