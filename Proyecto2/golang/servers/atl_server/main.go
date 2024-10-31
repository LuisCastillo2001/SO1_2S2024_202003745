package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"math/rand"
	"net"
	"time"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	pb "go-server/proto"
	"google.golang.org/grpc"
)

var (
	port        = flag.Int("port", 50051, "The server port")
	kafkaBroker = "my-cluster-kafka-bootstrap:9092"
	winnersTopic = "winners-topic"
	losersTopic  = "lossers-topic"
)

type server struct {
	pb.UnimplementedStudentServer
	producer *kafka.Producer
}

func (s *server) GetStudent(_ context.Context, in *pb.StudentRequest) (*pb.StudentResponse, error) {
	log.Printf("Received student: %v", in)

	
	studentData := map[string]interface{}{
		"name":       in.GetName(),
		"age":        in.GetAge(),
		"faculty":    in.GetFaculty(),
		"discipline": in.GetDiscipline(),
	}

	
	rand.Seed(time.Now().UnixNano())
	topic := winnersTopic
	if rand.Intn(2) == 0 {
		topic = losersTopic
	}

	
	messageBytes, err := json.Marshal(studentData)
	if err != nil {
		log.Printf("Failed to marshal request to JSON: %v", err)
		return nil, err
	}

	
	kafkaMessage := &kafka.Message{
		TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
		Value:          messageBytes,
	}
	err = s.producer.Produce(kafkaMessage, nil)
	if err != nil {
		log.Printf("Failed to produce message: %v", err)
		return nil, err
	}

	log.Printf("Message published to Kafka topic: %s", topic)
	return &pb.StudentResponse{
		Success: true,
	}, nil
}

func main() {
	flag.Parse()

	
	producer, err := kafka.NewProducer(&kafka.ConfigMap{"bootstrap.servers": kafkaBroker})
	if err != nil {
		log.Fatalf("Failed to create producer: %v", err)
	}
	defer producer.Close()

	// Configurar el listener gRPC
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	
	s := grpc.NewServer()
	studentService := &server{
		producer: producer,
	}
	pb.RegisterStudentServer(s, studentService)

	
	go func() {
		for e := range producer.Events() {
			switch ev := e.(type) {
			case *kafka.Message:
				if ev.TopicPartition.Error != nil {
					log.Printf("Failed to deliver message: %v", ev.TopicPartition)
				} else {
					log.Printf("Message delivered to topic %s [%d] at offset %v",
						*ev.TopicPartition.Topic, ev.TopicPartition.Partition, ev.TopicPartition.Offset)
				}
			}
		}
	}()

	log.Printf("Server started on port %d", *port)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
