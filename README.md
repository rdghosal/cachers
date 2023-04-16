# memoirs
> Because 'built in Rust' must mean it's blazing fast... right?

## Description

Rust that compiles into a Python package that offers simple in-**mem**ory caching...
or is it more like **mem**oization? 🤔

```python
import memoirs

@memoirs.Cache
def my_fancy_func(*args) -> str:
    print('running') 
    return ' '.join(str(a) for a in args)

```

Once a function return is memoized, subsequent invocations return a cached result without running the function again.

```python

>>> my_fancy_func('Hello', 'World!') 
running
'Hello World!'

>>> my_fancy_func('Hello', 'World!') 
'Hello World!'

```

For a more interesting example (assuming you like basic arithemtic), take a look at `examples/test.ipynb`.

## Installation

To play around with it (please don't use this in prod).

### 1. Set up a virtual environment.

```bash
python -m venv .venv

```

### 2. Install a dev version of the package into your virtual environment.

```bash
maturin develop

```

There. Good to go.

## But, how?

This project is made possible by `pyo3`. [Check 'em out!](https://github.com/PyO3)!

