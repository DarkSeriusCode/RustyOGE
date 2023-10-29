import os, sys
import urllib.request
from typing import List

URL = "https://inf-oge.sdamgia.ru/get_file?id={id}"
IDS = {
    12: (
        ("10504-10601.rar", 48458), ("11322.rar", 48494), ("11323.rar", 48495), ("11324.rar", 48496),
        ("11325.rar", 48497), ("11326.rar", 48498), ("11327.rar", 48499), ("11328.rar", 48500),
        ("11329.rar", 48501), ("11330.rar", 48502), ("11331.rar", 48503), ("18762.rar", 32913),
        ("18180.rar", 48512), ("18195.rar", 48515), ("18221.rar", 48517), ("18236.rar", 48520),
        ("18251.rar", 48523), ("18266.rar", 48526), ("18281.rar", 48528), ("18296.rar", 55346),
        ("16020.rar", 48508), ("18042.rar", 48510)
        )
}

def main(argv: List[str]):
    for (problem_num, ids) in IDS.items():
        print(f"Downloading test files for module {problem_num} to tests/module{problem_num}_files")
        folder = f"module{problem_num}_files"
        os.makedirs(folder, exist_ok=True)
        for (file_name, file_id) in ids:
            file_path = f"{folder}/{file_name}"
            print(f"\tDownloading {file_name}...", end="")
            sys.stdout.flush()
            if os.path.exists(file_path) and "--force" not in argv:
                print("Already downloaded! Skipping")
                continue
            urllib.request.urlretrieve(URL.format(id=file_id), file_path)
            print("Success!")

if __name__ == "__main__":
    main(sys.argv)
