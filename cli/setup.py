from setuptools import setup, find_packages

setup(
    name="traces-sm",
    version="0.1.0",
    packages=find_packages(),
    entry_points={
        "console_scripts": [
            "traces-sm = traces_sm.__main__:cli",
        ],
    },
)
