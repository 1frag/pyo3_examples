import time
import pytest
import asyncio
from pyo3_examples import async_sleep

pytestmark = [pytest.mark.asyncio]


@pytest.fixture(scope="session")
def event_loop():
    loop = asyncio.new_event_loop()
    yield loop
    loop.close()


@pytest.fixture(scope="session")
def tokio():
    async_sleep.init()


async def test_rust_sleep(event_loop, tokio):
    t0 = time.time()
    await async_sleep.fetch_db(1)
    assert 1 < time.time() - t0 < 2


async def test_many_rust_sleep_with_python_sleep(event_loop, tokio):
    t0 = time.time()
    await asyncio.gather(*(
            [async_sleep.fetch_db(1) for _ in range(5)] + [asyncio.sleep(1) for _ in range(5)]
    ))
    assert 1 < time.time() - t0 < 2
