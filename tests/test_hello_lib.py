from pyo3_examples import hello


def test_hello_world():
    assert hello.world() == 'hello world'
    assert hello.somebody('foo') == 'hello foo'
