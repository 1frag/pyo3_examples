from pyo3_examples import utils
import time


def py_fib(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


def rust_fib(n):
    return int(utils.nth_fib(n))


def test_fib_correct():
    for i in range(1_000):
        assert py_fib(i) == rust_fib(i)


def test_fib_faster():
    n = 100_000

    t0 = time.time()
    py_fib(n)
    dt1 = time.time() - t0

    t0 = time.time()
    rust_fib(n)
    dt2 = time.time() - t0

    assert dt1 > dt2


def test_fib_iter():
    for i, ith in enumerate(utils.Fib(), start=2):
        assert int(ith) == rust_fib(i)
        if i > 150:
            break


def test_str_xor():
    def str_xor(rhs, lhs):
        return bytes([x ^ y for x, y in zip(rhs, lhs)])

    def check(lhs, rhs):
        assert str_xor(lhs, rhs) == utils.str_xor(lhs, rhs)

    check(b"hello", b"world")
    check(b"1234", b"5462")
    check(b"1234", b"1234")
    check(b"bytes", b"strings")
