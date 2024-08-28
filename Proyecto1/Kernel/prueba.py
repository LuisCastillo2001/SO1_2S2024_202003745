import json

# Cargar el archivo JSON
with open('luis.json', 'r') as file:
    data = json.load(file)

# Recorrer la lista de procesos
for process in data['processes']:
    print(f"ID del contenedor: {process.get('id_container')}")
    print(f"PID: {process.get('PID')}")
    print(f"Nombre: {process.get('Nombre')}")

    # Verificar si el proceso tiene atributos de memoria
    if 'Memoria física' in process:
        print(f"Memoria física: {process['Memoria física']}")
        print(f"Memoria virtual: {process['Memoria virtual']}")
        print(f"Uso de memoria: {process['Uso de memoria']}")
    
    print("-" * 20)
