import hashlib
import os
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from time import perf_counter
from typing import Iterator, Tuple


def get_hash(path: str) -> Tuple[str, str]:
    sha = hashlib.sha256()
    with open(path, "rb") as f:
        for byte_block in iter(lambda: f.read(30 * 1024), b""):
            sha.update(byte_block)
    return path, sha.hexdigest()


def get_files(root_dir) -> Iterator[str]:
    for dir, _, files in os.walk(root_dir):
        for file in files:
            yield os.path.join(dir, file)


def main() -> None:
    root_dir = sys.argv[1]

    start = perf_counter()

    with ThreadPoolExecutor(4) as pool:
        futures = [pool.submit(get_hash, file) for file in get_files(root_dir)]
        for future in as_completed(futures):
            path, sha = future.result()
            print(f"{path}: {sha}", flush=True)

    end = perf_counter()

    print(f"Took {end - start} seconds to crawl {root_dir}")


if __name__ == "__main__":
    main()
