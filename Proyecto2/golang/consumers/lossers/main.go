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


func main() {
	
	config := &kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap:9092", 
		"group.id":          "losser-consumer-group",
		"auto.offset.reset": "earliest",
	}

	
	consumer, err := kafka.NewConsumer(config)
	if err != nil {
		log.Fatalf("Error creando consumidor: %v", err)
	}


	err = consumer.Subscribe("lossers-topic", nil)
	if err != nil {
		log.Fatalf("Error suscribiendo al tópico: %v", err)
	}

	log.Println("Esperando mensajes del tópico 'losser'...")

	ctx := context.Background()
	rdb := redis.NewClient(&redis.Options{
		Addr: "redis:6379", 
	})

	
	for {
		msg, err := consumer.ReadMessage(-1) 
		if err == nil {
			log.Printf("Mensaje recibido: %s", string(msg.Value))

			
			var student Student
			if err := json.Unmarshal(msg.Value, &student); err != nil {
				log.Printf("Error al deserializar el mensaje: %v", err)
				continue
			}

			
			facultyLower := strings.ToLower(student.Faculty)

			
			switch facultyLower {
			case "ingenieria":
				
				err := rdb.HIncrBy(ctx, "students", "ingenieria", 1).Err()
				if err != nil {
					log.Printf("Error al incrementar students:ingenieria: %v", err)
				} else {
					log.Println("Incrementado students:ingenieria en el hash")
				}

				err = rdb.HIncrBy(ctx, "lossers", "ingenieria", 1).Err()
				if err != nil {
					log.Printf("Error al incrementar lossers:ingenieria: %v", err)
				} else {
					log.Println("Incrementado lossers:ingenieria en el hash")
				}

			case "agronomia":
				
				err := rdb.HIncrBy(ctx, "students", "agronomia", 1).Err()
				if err != nil {
					log.Printf("Error al incrementar students:agronomia: %v", err)
				} else {
					log.Println("Incrementado students:agronomia en el hash")
				}

				err = rdb.HIncrBy(ctx, "lossers", "agronomia", 1).Err()
				if err != nil {
					log.Printf("Error al incrementar lossers:agronomia: %v", err)
				} else {
					log.Println("Incrementado lossers:agronomia en el hash")
				}

			default:
				log.Printf("Facultad %s no es válida\n", student.Faculty)
			}

		} else {
			log.Printf("Error al leer el mensaje: %v", err)
		}
	}
}