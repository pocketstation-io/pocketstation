#!/usr/bin/env python3
"""Run the PocketStation documentation completion authority."""

from documentation_compiler import main


if __name__ == "__main__":
    import sys

    sys.argv = [sys.argv[0], "verify", *sys.argv[1:]]
    main()
