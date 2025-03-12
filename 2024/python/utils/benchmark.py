# benchmark.py

import time


def benchmark(func):
    def wrapper(*args, **kwargs):
        start_time = time.perf_counter()
        result = func(*args, **kwargs)
        end_time = time.perf_counter()
        elapsed_time = (end_time - start_time) * 1_000_000  # Convert to microseconds
        print(f"Execution time of {func.__name__}: {elapsed_time:.2f} µs")
        return result

    return wrapper
