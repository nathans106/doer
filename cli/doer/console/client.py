from datetime import date
from typing import List, Optional

from doer import storage
from doer import config
from doer.model import Data, Priority
from doer.model.day_editor import DayEditor


def _collect_items(date_) -> List[Priority]:
    items: List[Priority] = []
    line = input(f"List items for {date_}\n")

    while line:
        items.append(Priority(line))
        line = input("Anymore?\n")

    return items


def _get_item_id(max_id) -> Optional[int]:
    while True:
        choice = input("Select the top priority, or press enter to end.\n")
        if choice is None:
            return None

        if not choice.isdigit():
            print(f"{choice} is not a digit, please try again")
            continue

        id_ = int(choice)
        if id_ > max_id:
            print(f"{id_} is too large, maximum is {max_id}")
            continue

        return id_


def _order_items(items: List[Priority]) -> List[Priority]:
    result: List[Priority] = []
    remaining = items

    for priority in range(6):
        max_id = len(remaining)
        if max_id == 0:
            return result

        print("Remaining:")
        for i, item in enumerate(remaining, start=1):
            print(f"{i}. {item.name}")

        choice = _get_item_id(max_id)
        if not choice:
            return result

        result.append(remaining.pop(int(choice) - 1))

    return result


class Client:
    def __init__(self):
        self._data = Data(storage.database(), config.context())
        self._day_editor = DayEditor(date.today(), self._data)

    @staticmethod
    def set_context(new_context):
        config.set_context(new_context)

    @staticmethod
    def contexts():
        for context_ in config.contexts():
            print(context_)

    @staticmethod
    def context():
        print(config.context())

    @property
    def last_date(self) -> date:
        return self._data.last_date

    def plan_priorities(self, date_: date):
        with self._edit_day(date_) as day:
            items: List[Priority] = []
            if len(day.priorities) > 0:
                self.show_priorities(date_)
                items.extend(day.priorities)

            items.extend(_collect_items(date_))
            day.priorities = _order_items(items)

        self.show_priorities(date_)

    def copy_priorities(self, date_from: date, date_to: date):
        with self._edit_day(date_to) as to_:
            from_ = self._data.day(date_from)
            to_.priorities = from_.priorities

        self.show_priorities(date_to)

    def show(self, date_: date):
        self.show_priorities(date_)
        self.show_log(date_)
        self.show_notes(date_)

    def show_priorities(self, date_: date):
        day = self._data.day(date_)

        if len(day.priorities) == 0:
            print(f"{date_} has no priorities")
            return

        focus = day.focus.name if day.focus is not None else ""

        print(f"Priorities for {date_} are:")
        for i, priority in enumerate(day.priorities, start=1):
            line = f"{i}. {priority.name}"

            if priority.name == focus:
                line += "*"
                focus = ""

            if priority.done:
                line += " - done"

            print(line)

        if focus != "":
            print(f"\nFocus: {focus}")

        print("\n")

    def show_log(self, date_: date):
        day = self._data.day(date_)

        if len(day.log) == 0:
            print(f"{date_} has no log")
            return

        print(f"Log for {date_} is:")

        for focus in day.log:
            start = focus.start.isoformat("minutes")
            print(f"{start} - {focus.name}")

            for break_ in focus.breaks:
                break_start = break_.start_time.isoformat("minutes")
                break_end = break_.end_time.isoformat("minutes")
                print(f"\t{break_start} - {break_end}")

        print("\n")

    def show_notes(self, date_: date):
        day = self._data.day(date_)

        if len(day.notes) == 0:
            print(f"{date_} has no notes")
            return

        print(f"Notes for {date_}:")

        for i, note in enumerate(day.notes, start=1):
            print(f"{i}. {note}")

        print("\n")

    def tick(self, id_: int):
        self._set_tick(id_, True)

    def un_tick(self, id_: int):
        self._set_tick(id_, False)

    def _set_tick(self, id_: int, state: bool):
        with self._edit_today() as day:
            max_id = len(day.priorities) - 1
            if id_ > max_id:
                print(f"id {id_} invalid, maximum is {max_id}")
                return

            day.priorities[id_].done = state

    def set_focus_to_priority(self, id_: int):
        with self._edit_today() as day:
            max_id = len(day.priorities) - 1
            if id_ > max_id:
                print(f"id {id_} invalid, maximum is {max_id}")
                return

            day.focus = day.priorities[id_].name

    def set_focus(self, name: str):
        with self._edit_today() as day:
            day.focus = name

    def start_break(self):
        with self._edit_today() as day:
            day.focus.start_break()

    def end_break(self):
        with self._edit_today() as day:
            day.focus.end_break()

    def end_day(self):
        with self._edit_today() as day:
            day.end()

    def note(self, note: str):
        with self._edit_today() as day:
            day.notes.append(note)

    def _edit_day(self, date_: date):
        return DayEditor(date_, self._data)

    def _edit_today(self):
        return self._edit_day(date.today())
