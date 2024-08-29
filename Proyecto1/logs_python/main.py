from fastapi import FastAPI # type: ignore
import os
import json
from typing import List
from models.models import LogProcess

app = FastAPI()


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/logs")
def get_logs(logs_proc: List[LogProcess]):
    logs_file = 'logs/logs.json'
    
    
    if os.path.exists(logs_file):
        
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        
        existing_logs = []

   
    new_logs = [log.dict() for log in logs_proc]
    existing_logs.extend(new_logs)

    
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}