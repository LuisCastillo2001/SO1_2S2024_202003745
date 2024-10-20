package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type RequestData struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline string `json:"discipline"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		return
	}

	var data RequestData
	err := json.NewDecoder(r.Body).Decode(&data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Mostrar la información recibida
	response := fmt.Sprintf("Nombre: %s, Edad: %d, Facultad: %s, Disciplina: %s", data.Name, data.Age, data.Faculty, data.Discipline)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"message": response})
}

func main() {
	http.HandleFunc("/receive", handler)
	port := ":8080"
	fmt.Printf("Servidor escuchando en el puerto %s\n", port)
	log.Fatal(http.ListenAndServe(port, nil))
}
