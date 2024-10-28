use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use studentgrpc::student_client::StudentClient;
use studentgrpc::StudentRequest;

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

    let mut client = match StudentClient::connect(format!("http://{}", server_addr)).await {
        Ok(client) => {
            println!("Successfully connected to the gRPC server at {}", server_addr);
            client
        },
        Err(e) => {
            println!("Failed to connect to gRPC server: {}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to connect to gRPC server: {}", e));
        }
    };

    let request = tonic::Request::new(StudentRequest {
        name: student.name.clone(),
        age: student.age,
        faculty: student.faculty.clone(),
        discipline: student.discipline,
    });

    println!("Sending gRPC request: {:?}", request);

    match client.get_student(request).await {
        Ok(response) => {
            println!("Received response from gRPC server: {:?}", response);
            HttpResponse::Ok().json(format!("Student: {:?}", response))
        },
        Err(e) => {
            println!("gRPC call failed: {}", e);
            HttpResponse::InternalServerError().body(format!("gRPC call failed: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/ingenieria", web::post().to(handle_student))
            .route("/agronomia", web::post().to(handle_student))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}