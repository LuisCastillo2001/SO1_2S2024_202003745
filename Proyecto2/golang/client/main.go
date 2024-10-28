package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	pb "client/proto" // Cambia esta ruta si es necesario para apuntar a tu paquete proto
)

// Direcciones de los servidores de disciplina
var serverAddresses = map[int]string{
	int(pb.Discipline_swimming):  "swimming-server:50051",
	int(pb.Discipline_athletics): "athletics-server:50051",
	int(pb.Discipline_boxing):    "boxing-server:50051",
}


// Modelo de datos del estudiante en el cliente
type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func getServerAddress(discipline int) (string, error) {
	address, ok := serverAddresses[discipline]
	if !ok {
		return "", fmt.Errorf("invalid discipline")
	}
	return address, nil
}

func sendData(fiberCtx *fiber.Ctx) error {
	var body Student
	if err := fiberCtx.BodyParser(&body); err != nil {
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	
	serverAddr, err := getServerAddress(body.Discipline)
	if err != nil {
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": "Invalid discipline provided",
		})
	}

	
	conn, err := grpc.NewClient(serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	
	c := pb.NewStudentClient(conn)

	
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	request := &pb.StudentRequest{
		Name:       body.Name,
		Age:        int32(body.Age),
		Faculty:    body.Faculty,
		Discipline: pb.Discipline(body.Discipline),
	}

	fmt.Println("Sending gRPC request:", request)

	response, err := c.GetStudent(ctx, request)
	if err != nil {
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return fiberCtx.JSON(fiber.Map{
		"message": response.GetSuccess(),
	})
}

func main() {
	app := fiber.New()
	app.Post("/ingenieria", sendData)
	fmt.Println("Client server running at :8080")
	if err := app.Listen(":8080"); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
