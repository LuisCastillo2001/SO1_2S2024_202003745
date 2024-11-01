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

// Configurar el consumidor de Kafka
func createKafkaConsumer() *kafka.Consumer {
	config := &kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap:9092",
		"group.id":          "winner-consumer-group",
		"auto.offset.reset": "earliest",
	}

	consumer, err := kafka.NewConsumer(config)
	if err != nil {
		log.Fatalf("Error creando consumidor: %v", err)
	}
	if err = consumer.Subscribe("winners-topic", nil); err != nil {
		log.Fatalf("Error suscribiendo al tópico: %v", err)
	}

	log.Println("Esperando mensajes del tópico 'winners-topic'...")
	return consumer
}

// Configurar cliente de Redis
func createRedisClient() *redis.Client {
	return redis.NewClient(&redis.Options{
		Addr: "redis:6379",
	})
}

// Procesar y manejar un mensaje
func processMessage(msg []byte, rdb *redis.Client, ctx context.Context) {
	var student Student
	if err := json.Unmarshal(msg, &student); err != nil {
		log.Printf("Error al deserializar el mensaje: %v", err)
		return
	}

	faculty := strings.ToLower(student.Faculty)
	if !incrementFacultyCounter(faculty, student.Student, rdb, ctx) {
		log.Printf("Facultad %s no es válida\n", student.Faculty)
	}

	if !incrementDisciplineCounter(student.Discipline, student.Student, rdb, ctx) {
		log.Printf("Disciplina %d no es válida para el ganador\n", student.Discipline)
	}
}

// Incrementar el contador en Redis para una facultad
func incrementFacultyCounter(faculty, studentName string, rdb *redis.Client, ctx context.Context) bool {
	switch faculty {
	case "ingenieria":
		log.Printf("¡Ganador de Ingeniería! Estudiante: %s\n", studentName)
		rdb.HIncrBy(ctx, "students", "ingenieria", 1)
	case "agronomia":
		log.Printf("¡Ganador de Agronomía! Estudiante: %s\n", studentName)
		rdb.HIncrBy(ctx, "students", "agronomia", 1)
	default:
		return false
	}
	return true
}

// Incrementar el contador en Redis para una disciplina
func incrementDisciplineCounter(discipline int, studentName string, rdb *redis.Client, ctx context.Context) bool {
	switch discipline {
	case 1:
		log.Printf("Disciplina de natación para el ganador: %s\n", studentName)
		rdb.HIncrBy(ctx, "winners", "natacion", 1)
	case 2:
		log.Printf("Disciplina de atletismo para el ganador: %s\n", studentName)
		rdb.HIncrBy(ctx, "winners", "atletismo", 1)
	case 3:
		log.Printf("Disciplina de boxeo para el ganador: %s\n", studentName)
		rdb.HIncrBy(ctx, "winners", "boxeo", 1)
	default:
		return false
	}
	return true
}

func main() {
	consumer := createKafkaConsumer()
	defer consumer.Close()

	rdb := createRedisClient()
	defer rdb.Close()

	ctx := context.Background()

	for {
		msg, err := consumer.ReadMessage(-1)
		if err == nil {
			log.Printf("Mensaje recibido: %s", string(msg.Value))
			processMessage(msg.Value, rdb, ctx)
		} else {
			log.Printf("Error al leer el mensaje: %v", err)
		}
	}
}
