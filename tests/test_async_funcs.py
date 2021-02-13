import asyncio
import time

from python_to_rust import async_funcs


async def speed_one(lang):
    if lang == 'rust':
        fn = lambda: async_funcs.sleep(1)
    else:
        fn = lambda: asyncio.sleep(1)

    t0 = time.time()
    for _ in range(5):
        await fn()
    return time.time() - t0


async def test_speed_sleep():
    """
    Check that rust's sleep not so slow
    * Actually, sometimes can be failed
    """
    speed_rust = await speed_one('rust')
    speed_python = await speed_one('python')

    assert speed_python - speed_rust > -0.001


async def test_asyncable_sleep():
    fn = lambda: async_funcs.sleep(1)
    t0 = time.time()
    await asyncio.gather(*[fn() for _ in range(300)])
    assert time.time() - t0 < 2
