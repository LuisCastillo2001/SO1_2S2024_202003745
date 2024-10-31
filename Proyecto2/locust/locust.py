from locust import HttpUser, TaskSet, task, between, events
import random

class StudentTaskSet(TaskSet):
    @task
    def send_requests(self):
        # Generar una lista de 1000 peticiones con facultades no equitativas
        total_requests = 1000
        
        # Definir una lista de facultades con probabilidades diferentes
        faculties = ["Ingenieria"] * 6 + ["Agronomia"] * 4  # 70% Ingenieria, 30% Agronomia

        for i in range(total_requests):
            payload = {
                "name": f"student_{i + 1}",  # Asegúrate de que los nombres sean únicos
                "age": random.randint(18, 30),
                "faculty": random.choice(faculties),  # Escoge la facultad aleatoriamente con la distribución
                "discipline": random.randint(1, 3)
            }

            if payload["faculty"] == "Agronomia":
                self.client.post("/agronomia", json=payload)
                #print("Enviando a Agronomía:", payload)
            else:
                self.client.post("/ingenieria", json=payload)
                #print("Enviando a Ingeniería:", payload)

class WebsiteUser(HttpUser):
    tasks = [StudentTaskSet]
    wait_time = between(1, 5)  # Espera de 1 a 5 segundos aleatoria entre solicitudes
    host = "http://35.243.225.168.nip.io"

@events.test_stop.add_listener
def on_test_stop(environment, **kwargs):
    print("Prueba de carga completada. Todas las solicitudes se han enviado.")

# Ejecutar el script con el comando:
# locust -f locust.py --headless -u 5 -r 1 --run-time 1m --host http://35.243.225.168.nip.io
