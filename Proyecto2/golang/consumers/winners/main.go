package main

import (
	"context"
	"encoding/json"
	"log"
	"strings"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/go-redis/redis/v8"
)

// Definir la estructura del mensaje
type Student struct {
	Student    string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func main() {
	config := &kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap:9092",
		"group.id":          "winner-consumer-group",
		"auto.offset.reset": "earliest",
	}

	// Crear consumidor de Kafka
	consumer, err := kafka.NewConsumer(config)
	if err != nil {
		log.Fatalf("Error creando consumidor: %v", err)
	}

	// Suscribirse al tópico
	err = consumer.Subscribe("winners-topic", nil)
	if err != nil {
		log.Fatalf("Error suscribiendo al tópico: %v", err)
	}

	log.Println("Esperando mensajes del tópico 'winners-topic'...")

	// Conectar a Redis
	ctx := context.Background()
	rdb := redis.NewClient(&redis.Options{
		Addr: "redis:6379", // Cambia si el host o puerto de Redis es diferente
	})

	// Bucle para consumir mensajes
	for {
		msg, err := consumer.ReadMessage(-1)
		if err == nil {
			log.Printf("Mensaje recibido: %s", string(msg.Value))

			// Deserializar el mensaje de Kafka
			var student Student
			if err := json.Unmarshal(msg.Value, &student); err != nil {
				log.Printf("Error al deserializar el mensaje: %v", err)
				continue
			}

			// Convertir la facultad a minúsculas
			facultyLower := strings.ToLower(student.Faculty)

			// Incrementar el contador en Redis para la facultad y disciplina
			switch facultyLower {
			case "ingenieria":
				log.Printf("¡Ganador de Ingeniería! Estudiante: %s\n", student.Student)
				rdb.HIncrBy(ctx, "students", "ingenieria", 1)
			case "agronomia":
				log.Printf("¡Ganador de Agronomía! Estudiante: %s\n", student.Student)
				rdb.HIncrBy(ctx, "students", "agronomia", 1)
			default:
				log.Printf("Facultad %s no es válida\n", student.Faculty)
			}

			
			switch student.Discipline {
			case 1:
				log.Printf("Disciplina de natación para el ganador: %s\n", student.Student)
				rdb.HIncrBy(ctx, "winners", "natacion", 1)
			case 2:
				log.Printf("Disciplina de atletismo para el ganador: %s\n", student.Student)
				rdb.HIncrBy(ctx, "winners", "atletismo", 1)
			case 3:
				log.Printf("Disciplina de boxeo para el ganador: %s\n", student.Student)
				rdb.HIncrBy(ctx, "winners", "boxeo", 1)
			default:
				log.Printf("Disciplina %d no es válida para el ganador\n", student.Discipline)
			}

		} else {
			
			log.Printf("Error al leer el mensaje: %v", err)
		}
	}
}
