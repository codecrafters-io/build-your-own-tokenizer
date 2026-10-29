import json
import sys


def main():
    command = sys.argv[1]
    if command == "bytes":
        with open(sys.argv[2], "rb") as f:
            data = f.read()
        print(json.dumps(list(data)))
    else:
        raise RuntimeError(f"Unknown command {command}")


if __name__ == "__main__":
    main()
