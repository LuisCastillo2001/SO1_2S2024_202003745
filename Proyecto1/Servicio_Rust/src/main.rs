use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::process::Command;
use chrono::Utc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "Total RAM")]
    total_ram: u64,
    #[serde(rename = "Free RAM")]
    free_ram: u64,
    #[serde(rename = "Shared RAM")]
    shared_ram: u64,
    #[serde(rename = "processes")]
    processes: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "id_container")]
    id_container: String,
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
    #[serde(rename = "VSZ")]
    vsz: f64,
    #[serde(rename = "RSS")]
    rss: f64,
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    vsz: f64,
    rss: f64,
    memory_usage: f64,
    cpu_usage: f64,
    action: String,
    timestamp: String,
}

impl Process {
    fn get_id_container(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}


impl Eq for Process {}  

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            
            .then_with(|| self.memory_usage.partial_cmp(&other.memory_usage).unwrap_or(std::cmp::Ordering::Equal))
            
            .then_with(|| self.vsz.partial_cmp(&other.vsz).unwrap_or(std::cmp::Ordering::Equal))
           
            .then_with(|| self.rss.partial_cmp(&other.rss).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn kill_container(id: &str) -> std::process::Output {
    let output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("rm")
        .arg("-f") // Forzar eliminación del contenedor
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Eliminando contenedor con id: {}", id);

    output
}

fn analyzer( system_info:  SystemInfo) {

    println!("Total RAM: {} KB", system_info.total_ram);
    println!("Free RAM: {} KB", system_info.free_ram);
    println!("Shared RAM: {} KB", system_info.shared_ram);
    println!("------------------------------");
    
    let mut log_proc_list: Vec<LogProcess> = Vec::new();


    
    let mut processes_list: Vec<Process> = system_info.processes;


    
    
    
    processes_list.retain(|process| process.cmd_line != "/usr/local/bin/python /usr/local/bin/fastapi run main.py --port 5000 ");
    processes_list.sort();
    //let (lowest_list, highest_list) = processes_list.split_at(processes_list.len() / 2);


    let mid = processes_list.len() / 2;
    let (lowest_slice, highest_slice) = processes_list.split_at(mid);
    let mut lowest_list = lowest_slice.to_vec();
    let mut highest_list = highest_slice.to_vec();
    

    // Si lowest_list tiene menos de 3 elementos, movemos el primer elemento de highest_list a lowest_list
    if lowest_list.len() < 3 && !highest_list.is_empty() {
        // Mover el primer elemento de highest_list a lowest_list
        let element_to_move = highest_list.remove(0);
        lowest_list.push(element_to_move);
        
        // Reordenar las listas después de la modificación
        lowest_list.sort();
        highest_list.sort();
    }


    
    
    
    println!("Bajo consumo");
    for process in &lowest_list {
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}, VSZ: {}, RSS: {}", process.pid, process.name, process.id_container, process.memory_usage, process.cpu_usage, process.vsz, process.rss);
    }

    println!("------------------------------");

    println!("Alto consumo");
    for process in &highest_list {
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}, VSZ: {}, RSS: {}", process.pid, process.name, process.id_container, process.memory_usage, process.cpu_usage, process.vsz, process.rss);
    }

    

    println!("------------------------------");

    /* 
        En la lista de bajo consumo, matamos todos los contenedores excepto los 3 primeros.
        antes 
        | 1 | 2 | 3 | 4 | 5 |

        después
        | 1 | 2 | 3 |
    */

    if lowest_list.len() > 3 {
        
        for process in lowest_list.iter().skip(3) {
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.id_container.to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                action: "Killed".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };
    
            log_proc_list.push(log_process.clone());

           
            let _output = kill_container(&process.id_container);

        }
    } 

    /* 
        En la lista de alto consumo, matamos todos los contenedores excepto los 2 últimos.
        antes 
        | 1 | 2 | 3 | 4 | 5 |

        después
                    | 4 | 5 |
    */
    if highest_list.len() > 2 {
        // Iteramos sobre los procesos en la lista de alto consumo.
        for process in highest_list.iter().take(highest_list.len() - 2) {
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.id_container.to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                action: "Killed".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };
            log_proc_list.push(log_process.clone());

            
            let _output = kill_container(&process.id_container);

        }
    }

    

    let logs_json = match serde_json::to_string(&log_proc_list) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error converting logs to JSON: {}", e);
            return;
        }
    };

    
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -X POST http://localhost:5000/logs -H 'Content-Type: application/json' -d '{}'",
            logs_json
        ))
        .output()
        .expect("Failed to execute curl command");

    if output.status.success() {
        println!("Logs enviados exitosamente a la API.");
    } else {
        eprintln!("Error al enviar logs: {:?}", output.stderr);
    }
  
    
    
    println!("Contenedores matados");
    for process in log_proc_list {
        println!("PID: {}, Name: {}, Container ID: {}, Memory Usage: {}, CPU Usage: {} \n", process.pid, process.name, process.container_id,  process.memory_usage, process.cpu_usage);
    }
        

    println!("------------------------------");

    

    
}

fn eliminar_docker_compose() {
    let output = Command::new("sudo")
        .arg("docker")
        .arg("rm")
        .arg("-f") // Forzar eliminación del contenedor
        .arg("python_container")
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        println!("Contenedor 'python_container' eliminado con éxito.");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error al eliminar el contenedor: {}", stderr);
    }
    
}

fn execute_docker_compose() {
    let output = Command::new("docker-compose")
        .arg("-f")
        .arg("/home/cluiis/Documentos/SO1_2S2024_202003745/Proyecto1/logs_python/docker-compose.yml")
        .arg("up")
        .arg("-d") 
        .output()
        .expect("failed to execute docker-compose");

    if output.status.success() {
        println!("Servicios levantados exitosamente con docker-compose.");
    } else {
        eprintln!("Error ejecutando docker-compose: {:?}", output.stderr);
    }
    thread::sleep(Duration::from_secs(4));
}


fn read_proc_file(file_name: &str) -> io::Result<String> {
  
    let path  = Path::new("/proc").join(file_name);

   
    let mut file = File::open(path)?;

    
    let mut content = String::new();

    
    file.read_to_string(&mut content)?;


    
    Ok(content)
}


fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
   
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

   
    Ok(system_info)
}


fn send_memory_data(system_info: &SystemInfo) {
    let memory_json = match serde_json::to_string(&json!({
        "total_ram": system_info.total_ram,
        "free_ram": system_info.free_ram,
        "shared_ram": system_info.shared_ram,
    })) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error converting memory data to JSON: {}", e);
            return;
        }
    };

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -X POST http://localhost:5000/memory -H 'Content-Type: application/json' -d '{}'",
            memory_json
        ))
        .output()
        .expect("Failed to execute curl command");

    
}

fn generar_grafica() {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("curl -X GET http://localhost:5000/graficar")
        .output()
        .expect("Failed to execute curl command");

    if output.status.success() {
        println!("La solicitud para generar gráficos se realizó exitosamente.");
    } else {
        eprintln!("Error al realizar la solicitud para generar gráficos: {:?}", output.stderr);
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    
    ctrlc::set_handler(move || {
        generar_grafica();
        eliminar_docker_compose();
        
        println!("Deteniendo el servicio de Rust");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    execute_docker_compose();

    while running.load(Ordering::SeqCst) {
        let json_str = match read_proc_file("sysinfo_202003745") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading proc file: {}", e);
                continue; 
            }
        };

        let system_info = match parse_proc_to_struct(&json_str) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                continue; 
            }
        };
        // Aqui iria la peticion
        send_memory_data(&system_info);
        analyzer(system_info);
        
        
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}