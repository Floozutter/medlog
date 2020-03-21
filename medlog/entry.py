from datetime import datetime
from typing import NamedTuple, Callable, List, ClassVar


class EntryComponent:
    def stringify(self) -> str:
        raise NotImplementedError()

class Timestamp(EntryComponent):
    dt: datetime
    def __init__(self, dt: datetime):
        self.dt = dt

        
class Requirement(NamedTuple):
    test: Callable[[str], bool]
    issue: str
class Validation(NamedTuple):
    validity: bool
    issues: List[str]

class WrittenComponent(EntryComponent):
    reqs: ClassVar[List[Requirement]] = []
    writ: str
    @staticmethod
    def validate(writ: str) -> Validation:
        raise NotImplementedError()
