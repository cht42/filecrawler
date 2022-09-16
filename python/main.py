import hashlib
import os
import sys
from multiprocessing.pool import Pool
from time import perf_counter
from typing import Iterator, Optional, Tuple


def get_hash(path: str) -> Optional[Tuple[str, str]]:
    sha = hashlib.sha256()
    try:
        with open(path, "rb") as f:
            for byte_block in iter(lambda: f.read(30 * 1024), b""):
                sha.update(byte_block)
        return path, sha.hexdigest()
    except Exception:
        return None


def get_files(root_dir) -> Iterator[str]:
    for dir, _, files in os.walk(root_dir):
        for file in files:
            yield os.path.join(dir, file)


def main() -> None:
    if len(sys.argv) != 2:
        print("Wrong number of arguments")
        sys.exit(1)

    root_dir = sys.argv[1]

    start = perf_counter()

    with Pool(8) as pool:
        for res in pool.imap_unordered(get_hash, get_files(root_dir)):
            if not res:
                continue
            path, sha = res
            print(f"{path}: {sha}", flush=True)

    end = perf_counter()

    print(f"Took {end - start} seconds to crawl {root_dir}")


if __name__ == "__main__":
    main()
