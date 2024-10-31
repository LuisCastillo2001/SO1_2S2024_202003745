use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use studentgrpc::student_client::StudentClient;
use studentgrpc::StudentRequest;
use tokio::task;
use tokio::sync::oneshot;

pub mod studentgrpc {
    tonic::include_proto!("confproto");
}

#[derive(Deserialize, Serialize, Debug)]
struct StudentData {
    name: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

lazy_static::lazy_static! {
    static ref SERVER_ADDRESSES: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "swimming-server:50051"); // pb.Discipline_swimming
        m.insert(2, "athletics-server:50051"); // pb.Discipline_athletics
        m.insert(3, "boxing-server:50051"); // pb.Discipline_boxing
        m
    };
}

fn get_server_address(discipline: i32) -> Result<&'static str, &'static str> {
    match SERVER_ADDRESSES.get(&discipline) {
        Some(&address) => Ok(address),
        None => Err("Invalid discipline"),
    }
}

async fn handle_student(student: web::Json<StudentData>) -> impl Responder {
    println!("Received student data: {:?}", student);

    let server_addr = match get_server_address(student.discipline) {
        Ok(addr) => addr,
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::BadRequest().body(err);
        }
    };

    // Crear un canal para recibir la respuesta del thread
    let (tx, rx) = oneshot::channel();

    // Clonar los datos del estudiante para moverlos al thread
    let student_data = student.into_inner();

    // Crear un thread para manejar la solicitud gRPC
    task::spawn(async move {
        let mut client = match StudentClient::connect(format!("http://{}", server_addr)).await {
            Ok(client) => {
                println!("Successfully connected to the gRPC server at {}", server_addr);
                client
            },
            Err(e) => {
                println!("Failed to connect to gRPC server: {}", e);
                let _ = tx.send(Err(format!("Failed to connect to gRPC server: {}", e)));
                return;
            }
        };

        let request = tonic::Request::new(StudentRequest {
            name: student_data.name.clone(),
            age: student_data.age,
            faculty: student_data.faculty.clone(),
            discipline: student_data.discipline,
        });

        println!("Sending gRPC request: {:?}", request);

        match client.get_student(request).await {
            Ok(response) => {
                println!("Received response from gRPC server: {:?}", response);
                let _ = tx.send(Ok(response));
            },
            Err(e) => {
                println!("gRPC call failed: {}", e);
                let _ = tx.send(Err(format!("gRPC call failed: {}", e)));
            }
        }
    });

    // Esperar la respuesta del thread
    match rx.await {
        Ok(Ok(response)) => HttpResponse::Ok().json(format!("Student: {:?}", response)),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e),
        Err(_) => HttpResponse::InternalServerError().body("Failed to receive response from thread"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/ingenieria", web::post().to(handle_student))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}