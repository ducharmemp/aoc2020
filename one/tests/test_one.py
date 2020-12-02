import os

import pytest
from click.testing import CliRunner

from one import __version__
from one.solution import find_candidate, run, NotFound

INPUT_FILE_PATH = os.path.join(os.path.dirname(__file__), "../input.txt")


def test_version():
    assert __version__ == "0.1.0"


@pytest.fixture
def runner():
    return CliRunner()


def test_find_candidate():
    result = find_candidate(
        input_records=[
            1,
            2,
            3,
            4,
        ],
        target_sum=7,
        num_entries=2,
    )
    assert result == (3, 4)


def test_find_candidate_not_found():
    with pytest.raises(NotFound):
        find_candidate(
            input_records=[
                1,
                2,
                3,
                4,
            ],
            target_sum=8,
            num_entries=2,
        )


def test_two_entries_command(runner):
    result = runner.invoke(run, [INPUT_FILE_PATH])
    assert result.exit_code == 0
    assert result.output == "788739\n"


def test_three_entries_command(runner):
    result = runner.invoke(run, [INPUT_FILE_PATH, "--num-entries=3"])
    assert result.exit_code == 0
    assert result.output == "178724430\n"
