import pytest
from calculate import calculate, tokenize


def test_tokenize():
    assert tokenize("1") == ["1"]
    assert tokenize("11") == ["11"]
    assert tokenize("+") == ["+"]
    assert tokenize("1+1") == ["1", "+", "1"]
    assert tokenize("11+1") == ["11", "+", "1"]
    assert tokenize("+++") == ["+++"]
    assert tokenize("11+1-432/9") \
            == ["11", "+", "1", "-", "432", "/", "9"]


def test_add():
    assert calculate("0+0") == "0"
    assert calculate("0+1") == "1"
    assert calculate("1+0") == "1"
    assert calculate("1+1") == "2"
    assert calculate("5+2") == "7"
    assert calculate("6+9") == "15"


def test_sub():
    assert calculate("0-0") == "0"
    assert calculate("1-0") == "1"
    assert calculate("0-1") == "-1"
    assert calculate("5-2") == "3"
    assert calculate("4-9") == "-5"


def test_mul():
    assert calculate("5*5") == "25"


def test_div():
    assert calculate("8$2") == "4"


def test_multiple_with_same_priority():
    assert calculate("1+2+3+4") == "10"
    assert calculate("10-4-3") == "3"
    assert calculate("800$8$10") == "10"


def test_multiple_with_different_priority():
    assert calculate("5+5*5") == "30"
    assert calculate("5+5*5*2") == "55"
    assert calculate("5-5+5*5*2") == "50"
    assert calculate("1000$2.5$5+5-5+6$6") == "81"
    assert calculate("5+8-8*2$4") == "9"


def test_float():
    assert calculate("4.2+3.14*2") == "10.48"


def test_calculate_with_one_number():
    assert calculate("1") == "1"
    assert calculate("42") == "42"


def test_invalid_input():
    assert calculate("aaa") == "400: Bad request"
    assert calculate("2+2p") == "400: Bad request"
    assert calculate("2++2") == "400: Bad request"
    assert calculate("2+2+p") == "400: Bad request"
