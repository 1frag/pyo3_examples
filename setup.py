from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="python_to_rust",
    version="0.1.0",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=[],
    rust_extensions=[
        RustExtension('python_to_rust.async_funcs', "async_funcs/Cargo.toml", debug=False),
        RustExtension('python_to_rust.hello', "hello/Cargo.toml", debug=False),
        RustExtension('python_to_rust.pet_farm', "pet_farm/Cargo.toml", debug=False),
        RustExtension('python_to_rust.utils', "utils/Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
