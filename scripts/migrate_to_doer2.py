import json
from pathlib import Path
from typing import Dict

from appdirs import AppDirs


def main():
    dirs1 = AppDirs(appname="doer", appauthor="Nathan Samra")
    database_path1 = Path(dirs1.user_data_dir)
    dirs2 = AppDirs(appname="doer", appauthor="Nathan Samra", roaming=True)
    database_path2 = Path(dirs2.user_data_dir) / "data"

    context_path: Path
    for context_path in database_path1.iterdir():
        if context_path.is_file():
            continue

        context = context_path.name
        new_context = database_path2 / context
        new_context.mkdir(parents=True, exist_ok=True)
        year_file: Path

        for year_file in context_path.iterdir():
            year = {}
            with year_file.open("r") as f:
                year = json.load(f)

            new_year = {"version": "2.0.0", "days": year["days"]}
            new_year_file = new_context / year_file.name
            with new_year_file.open("w") as f:
                json.dump(new_year, f)


if __name__ == "__main__":
    main()
