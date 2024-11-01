package main

import (
	"context"
	"encoding/json"
	"log"
	"strings"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/go-redis/redis/v8"
)

type Student struct {
	Student    string `json:"student"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}


func createKafkaConsumer() *kafka.Consumer {
	kafkaConfig := &kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap:9092",
		"group.id":          "losser-consumer-group",
		"auto.offset.reset": "earliest",
	}

	consumer, err := kafka.NewConsumer(kafkaConfig)
	if err != nil {
		log.Fatalf("Error al crear consumidor: %v", err)
	}
	err = consumer.Subscribe("lossers-topic", nil)
	if err != nil {
		log.Fatalf("Error al suscribirse al tópico: %v", err)
	}
	log.Println("Esperando mensajes del tópico 'lossers-topic'...")
	return consumer
}


func createRedisClient() *redis.Client {
	return redis.NewClient(&redis.Options{
		Addr: "redis:6379",
	})
}


func processMessage(msg []byte, redisClient *redis.Client, ctx context.Context) {
	var student Student
	if err := json.Unmarshal(msg, &student); err != nil {
		log.Printf("Error al deserializar el mensaje: %v", err)
		return
	}

	faculty := strings.ToLower(student.Faculty)
	hashMap := map[string]string{"students": faculty, "lossers": faculty}

	for hashKey, field := range hashMap {
		if err := redisClient.HIncrBy(ctx, hashKey, field, 1).Err(); err != nil {
			log.Printf("Error al incrementar %s:%s en Redis: %v", hashKey, field, err)
		} else {
			log.Printf("Incrementado %s:%s en Redis", hashKey, field)
		}
	}
}

func main() {
	consumer := createKafkaConsumer()
	defer consumer.Close()

	redisClient := createRedisClient()
	defer redisClient.Close()

	ctx := context.Background()

	for {
		msg, err := consumer.ReadMessage(-1)
		if err != nil {
			log.Printf("Error al leer el mensaje: %v", err)
			continue
		}

		log.Printf("Mensaje recibido: %s", string(msg.Value))
		processMessage(msg.Value, redisClient, ctx)
	}
}
