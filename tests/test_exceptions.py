from pyo3_examples.exceptions import (
    MyFirstException,
    MyExceptionBasedOnKeyError,
    func_that_raise_value_error_42,
    ignore_value_error,
)


def test_first_exception():
    n = 0
    try:
        raise MyFirstException()
    except MyFirstException:
        n += 1
    assert n == 1


def test_inheritance_in_exception():
    n = 0
    try:
        raise MyExceptionBasedOnKeyError()
    except KeyError:
        n += 1

    assert n == 1


def test_func_that_raise_value_error_42():
    n = 0
    try:
        func_that_raise_value_error_42()
    except ValueError as e:
        assert e.args[0] == 42
        n += 1

    assert n == 1


def test_ignore_exception():
    ignore_value_error(func_that_raise_value_error_42)
