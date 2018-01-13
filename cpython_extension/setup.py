from setuptools import setup
# NOTE: pip3 install setuptools_rust
from setuptools_rust import Binding, RustExtension

setup(name='foo',
      version='1.0',
      rust_extensions=[RustExtension('foo._foo',
                                     'Cargo.toml', binding=Binding.PyO3)],
      packages=['foo'],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False
)
