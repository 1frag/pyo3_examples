from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pyo3_examples",
    version="0.1.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    rust_extensions=[
        RustExtension('pyo3_examples.async_funcs', "async_funcs/Cargo.toml", debug=False),
        RustExtension('pyo3_examples.hello', "hello/Cargo.toml", debug=False),
        RustExtension('pyo3_examples.pet_farm', "pet_farm/Cargo.toml", debug=False),
        RustExtension('pyo3_examples.utils', "utils/Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
