use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::process::Command;


#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "processes")]
    processes: Vec<Process>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    id_container: String,
    name: String,
    memory_usage: f64,
    cpu_usage: f64,
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


    
    let mut log_proc_list: Vec<LogProcess> = Vec::new();


    
    let mut processes_list: Vec<Process> = system_info.processes;


    /* 
        Cuando llamas a la función sort en un vector de Process, se ejecutarán los traits 
        Ord y PartialOrd en el siguiente orden y con la siguiente funcionalidad:


        La función sort del vector llama internamente a partial_cmp para comparar los elementos.
        partial_cmp delega la comparación a cmp del trait Ord.


        Comparación con cmp:

        cmp compara primero el uso de CPU (cpu_usage).
        Si el uso de CPU es igual, compara el uso de memoria (memory_usage).
        Si ambos son iguales, devuelve Ordering::Equal.
        Funcionalidad de los Traits
        PartialOrd: Permite la comparación parcial, necesaria para manejar casos donde los valores pueden ser NaN.
        Ord: Proporciona una comparación total, necesaria para ordenar completamente los elementos del vector.

        Cuando llamas a processes_list.sort(), el método sort usará partial_cmp y cmp para comparar y 
        ordenar los procesos en el vector processes_list basándose en el uso de CPU y memoria.
    */
    
    
    processes_list.retain(|process| process.cmd_line != "/usr/local/bin/python /usr/local/bin/fastapi run main.py --port 5000 ");
    processes_list.sort();
    //let (lowest_list, highest_list) = processes_list.split_at(processes_list.len() / 2);


   
    let (lowest_list, highest_list) = processes_list.split_at(processes_list.len() / 2);

    
    
    
    println!("Bajo consumo");
    for process in lowest_list {
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.id_container, process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    println!("Alto consumo");
    for process in highest_list {
        println!("PID: {}, Name: {}, Icontainer ID {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name,process.id_container,process.memory_usage, process.cpu_usage);
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
        // Iteramos sobre los procesos en la lista de bajo consumo.
        for process in lowest_list.iter().skip(3) {
            let log_process = LogProcess {
                pid: process.pid,
                id_container: process.id_container.to_string(),
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
            };
    
            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            //let _output = kill_container(&process.id_container);

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
                id_container: process.id_container.to_string(),
                name: process.name.clone(),
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage
            };
    
            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            //let _output = kill_container(&process.id_container);

        }
    }

    // TODO: ENVIAR LOGS AL CONTENEDOR REGISTRO

    // Hacemos un print de los contenedores que matamos.
    /*
    println!("Contenedores matados");
    for process in log_proc_list {
        println!("PID: {}, Name: {}, Container ID: {}, Memory Usage: {}, CPU Usage: {} \n", process.pid, process.name, process.id_container,  process.memory_usage, process.cpu_usage);
    }
        */

    println!("------------------------------");

    
}
fn execute_docker_compose() {
    let output = Command::new("docker-compose")
        .arg("-f")
        .arg("/home/cluiis/Documentos/SO1_2S2024_202003745/Proyecto1/logs_python/docker-compose.yml")
        .arg("up")
        .arg("-d")  // Levanta los servicios en modo "desprendido" (background)
        .output()
        .expect("failed to execute docker-compose");

    if output.status.success() {
        println!("Servicios levantados exitosamente con docker-compose.");
    } else {
        eprintln!("Error ejecutando docker-compose: {:?}", output.stderr);
    }
}

/*  
    Función para leer el archivo proc
    - file_name: El nombre del archivo que se quiere leer.
    - Regresa un Result<String> que puede ser un error o el contenido del archivo.
*/
fn read_proc_file(file_name: &str) -> io::Result<String> {
    // Se crea un Path con el nombre del archivo que se quiere leer.
    let path  = Path::new("/proc").join(file_name);

    /* 
        Se abre el archivo en modo lectura y se guarda en la variable file.
        En caso de que haya un error al abrir el archivo, se regresa un error.
        El signo de interrogación es un atajo para regresar un error en caso de que haya uno.
    */
    let mut file = File::open(path)?;

    // Se crea una variable mutable content que se inicializa con un String vacío.
    let mut content = String::new();

    // Se lee el contenido del archivo y se guarda en la variable content.
    file.read_to_string(&mut content)?;


    // Se regresa el contenido del archivo.
    Ok(content)
}

/* 
    Función para deserializar el contenido del archivo proc a un vector de procesos.
    - json_str: El contenido del archivo proc en formato JSON.
    - Regresa un Result<> que puede ser un error o un SystemInfo.
*/
fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    // Se deserializa el contenido del archivo proc a un SystemInfo.
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

    // Se regresa el SystemInfo.
    Ok(system_info)
}



fn main() {
    

 
    execute_docker_compose();
    
    // Continuar con el flujo normal del programa
    // TODO: antes de iniciar el loop, ejecutar el docker-compose.yml y obtener el id del contenedor registro.
    
    // TODO: Utilizar algo para capturar la señal de terminación y matar el contenedor registro y cronjob.
    // TODO: antes de iniciar el loop, ejecutar el docker-compose.yml y obtener el id del contenedor registro.

    // TODO: Utilizar algo para capturar la señal de terminación y matar el contenedor registro y cronjob.

    loop {
        
        // Creamos una estructura de datos SystemInfo con un vector de procesos vacío.
        let system_info: Result<SystemInfo, _>;

        // Leemos el contenido del archivo proc y lo guardamos en la variable json_str.
        let json_str = read_proc_file("sysinfo_202003745").unwrap();

        // Deserializamos el contenido del archivo proc a un SystemInfo.
        system_info = parse_proc_to_struct(&json_str);

        // Dependiendo de si se pudo deserializar el contenido del archivo proc o no, se ejecuta una u otra rama.
        match system_info {
            Ok( info) => {
                analyzer(info);
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        // Dormimos el hilo principal por 10 segundos.
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

}