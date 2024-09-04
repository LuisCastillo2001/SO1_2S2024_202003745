from fastapi import FastAPI
import os
import json
from typing import List
import matplotlib.pyplot as plt
import pandas as pd
from models.models import LogProcess, Memory


app = FastAPI()

def load_json_file(file_path):
    with open(file_path, 'r') as file:
        return json.load(file)

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


@app.post("/memory")
def update_memory(memory: Memory):
    memory_file = 'logs/logs_mem.json'

    
    with open(memory_file, 'w') as file:
        json.dump(memory.dict(), file, indent=4)

    return {"received": True}



@app.get("/graficar")
def generate_graphs():
    
    process_data = load_json_file('logs/logs.json')
    system_info = load_json_file('logs/logs_mem.json')

  
    df = pd.DataFrame(process_data)

  
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))

  
    df[['cpu_usage']].plot(kind='bar', ax=ax1, color='blue', legend=False)
    ax1.set_title('CPU Usage')
    ax1.set_xlabel('Process')
    ax1.set_ylabel('CPU Usage (%)')
    ax1.set_xticks(range(len(df)))
    ax1.set_xticklabels(range(1, len(df) + 1), rotation=0)

    
    df[['memory_usage']].plot(kind='bar', ax=ax2, color='green', legend=False)
    ax2.set_title('Memory Usage')
    ax2.set_xlabel('Process')
    ax2.set_ylabel('Memory Usage (%)')
    ax2.set_xticks(range(len(df)))
    ax2.set_xticklabels(range(1, len(df) + 1), rotation=0)
    ax2.set_ylim(0, 4)  


    plt.tight_layout()
    plt.savefig('logs/cpu_memory_usage.png')
    plt.close(fig)

 
    labels = 'Free RAM', 'Used RAM', 'Shared RAM'
    sizes = [system_info['free_ram'], system_info['total_ram'] - system_info['free_ram'], system_info['shared_ram']]
    fig, ax = plt.subplots()
    ax.pie(sizes, labels=labels, autopct='%1.1f%%', startangle=90)
    ax.axis('equal')  
    plt.title('System Memory Distribution')
    plt.savefig('logs/system_memory_distribution.png')
    plt.close(fig)

    return {"message": "Graphs generated and saved as images."}