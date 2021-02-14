from pyo3_examples import decorators


def test_double_decorator():
    @decorators.double
    def simple():
        return 21

    assert simple() == 42


def test_double_with_args():
    @decorators.double
    def with_args(*args):
        return sum(args)

    assert with_args(1, 2, 3) == 12
    assert with_args(*range(5)) == 20


def test_double_with_args_kwargs():
    @decorators.double
    def with_kwargs(*args, **kwargs):
        return len(args) + len(kwargs)

    assert with_kwargs(*range(5), **{'1': 2, '3': None}) == 14
