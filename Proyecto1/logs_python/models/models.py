from pydantic import BaseModel # type: ignore
from typing import List

class LogProcess(BaseModel):
    pid: int
    container_id: str
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    action: str
    timestamp: str

class Memory(BaseModel):
    total_ram: int
    free_ram: int
    shared_ram: int