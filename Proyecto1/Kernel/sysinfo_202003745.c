#include <linux/module.h>
#include <linux/kernel.h> //trae todas las funciones que se pueden usar en el kernel
#include <linux/init.h>
#include <linux/proc_fs.h> // trae las funciones para crear archivos en /proc
#include <linux/seq_file.h> // trae las funciones para escribir en archivos en /proc
#include <linux/mm.h> // trae las funciones para manejar la memoria
#include <linux/sched.h> // trae las funciones para manejar los procesos
#include <linux/timer.h> // trae las funciones para manejar los timers
#include <linux/jiffies.h> // trae las funciones para manejar los jiffies, que son los ticks del sistema
#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/uaccess.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Luis Castillo");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y CPU");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_202003745" // nombre del archivo en /proc, y así es como se crea la macro
#define MAX_CMDLINE_LENGTH 4096
#define CONTAINER_ID_LENGTH 12
static char *get_container_id(const char *cmdline) {
    char *id_start = strstr(cmdline, "-id "); // Busca la cadena "-id "
    if (id_start) {
        id_start += 4; // Avanza para saltar "-id "
        
        // Extrae los primeros 12 caracteres
        char *container_id = kmalloc(CONTAINER_ID_LENGTH + 1, GFP_KERNEL);
        if (container_id) {
            strncpy(container_id, id_start, CONTAINER_ID_LENGTH);
            container_id[CONTAINER_ID_LENGTH] = '\0'; // Termina la cadena
            return container_id;
        }
    }
    return NULL;
}


// Función para obtener la línea de comandos de un proceso
static char *get_process_cmdline(struct task_struct *task) {
    struct mm_struct *mm;
    char *cmdline, *p;
    unsigned long arg_start, arg_end, env_start;
    int i, len;

    cmdline = kmalloc(MAX_CMDLINE_LENGTH, GFP_KERNEL);
    if (!cmdline)
        return NULL;

    mm = get_task_mm(task);
    if (!mm) {
        kfree(cmdline);
        return NULL;
    }

    down_read(&mm->mmap_lock);
    arg_start = mm->arg_start;
    arg_end = mm->arg_end;
    env_start = mm->env_start;
    up_read(&mm->mmap_lock);

    len = arg_end - arg_start;

    if (len > MAX_CMDLINE_LENGTH - 1)
        len = MAX_CMDLINE_LENGTH - 1;

    if (access_process_vm(task, arg_start, cmdline, len, 0) != len) {
        mmput(mm);
        kfree(cmdline);
        return NULL;
    }

    cmdline[len] = '\0';

    // Reemplazar caracteres nulos por espacios
    p = cmdline;
    for (i = 0; i < len; i++)
        if (p[i] == '\0')
            p[i] = ' ';

    mmput(mm);
    return cmdline;
}

static int sysinfo_show(struct seq_file *m, void *v) {

    struct sysinfo si;
    struct task_struct *task;
    si_meminfo(&si); 
    unsigned long total_jiffies = jiffies;
    int first_process = 1;

    // Inicio del JSON
    seq_printf(m, "{\n");
    seq_printf(m, " \"Total RAM\": %lu  ,\n", si.totalram * 4);
    seq_printf(m, " \"Free RAM\": %lu  ,\n", si.freeram * 4);
    seq_printf(m, " \"Shared RAM\": %lu  ,\n", si.sharedram * 4);


    seq_printf(m, " \"processes\": [\n");
     // Para controlar la coma al inicio de cada objeto

    for_each_process(task) {
        char *cmdline = get_process_cmdline(task);
        
        if (strcmp(task->comm, "containerd-shim") == 0) {
           
            
            if (cmdline) {
                char *container_id = get_container_id(cmdline);
                if (strcmp(container_id, "46138ed5257b") != 0) {
                    if (!first_process) {
                        seq_printf(m, ",\n");
                    } else {
                        first_process = 0;
                    }

                    // JSON del proceso containerd-shim
                    seq_printf(m, "     {\n");
                    seq_printf(m, "     \"id_container\": \"%s\",\n", container_id);
                    seq_printf(m, "     \"PID\": %d,\n", task->pid);
                    seq_printf(m, "     \"Name\": \"%s\",\n", task->comm);
                    
                    //seq_printf(m, "      \"Nombre\": \"%s\",\n", task->comm);

                    // Liberar la memoria asignada
                    kfree(container_id);
                }else{
                    kfree(cmdline);
                    kfree(container_id);
                    continue;
                    
                }
                kfree(cmdline);
            }
            
        }

        if (strcmp(task->comm, "python3") == 0 || strcmp(task->comm, "fastapi") == 0) {
            unsigned long vsz = 0;
            unsigned long rss = 0;
            unsigned long totalram = si.totalram * 4;
            unsigned long mem_usage = 0;
            unsigned long cpu_usage = 0;
            char *cmdline = NULL;

            // Obtenemos los valores de VSZ y RSS
            if (task->mm) {
                // Obtenemos el uso de vsz haciendo un shift de PAGE_SHIFT - 10, un PAGE_SHIFT es la cantidad de bits que se necesitan para representar un byte
                vsz = task->mm->total_vm << (PAGE_SHIFT - 10);
                // Obtenemos el uso de rss haciendo un shift de PAGE_SHIFT - 10
                rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);
                // Obtenemos el uso de memoria en porcentaje
                mem_usage = (rss * 10000) / totalram;
            }

            /* 
                Obtenemos el tiempo total de CPU de un proceso
                Obtenemos el tiempo total de CPU de todos los procesos
                Obtenemos el uso de CPU en porcentaje
                Obtenemos la línea de comandos de un proceso
            */
            unsigned long total_time = task->utime + task->stime;
            cpu_usage = (total_time * 10000) / total_jiffies;
            cmdline = get_process_cmdline(task);

            if (strcmp(cmdline, "/usr/local/bin/python /usr/local/bin/fastapi run main.py --port 5000 ") == 0){
                kfree(cmdline);
                continue;
            }
           

           
            
            seq_printf(m, "     \"Cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "     \"MemoryUsage\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "     \"CPUUsage\": %lu.%02lu\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "     }\n");


            // Liberamos la memoria de la línea de comandos
            if (cmdline) {
                kfree(cmdline);
            }
        }
    }

    //seq_printf(m, "     { \n\"id_container\": \"000\" \n}");

    // Fin del JSON
    seq_printf(m, "\n  ]\n}\n");

    return 0;
}


 



static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}
//single_open: se encarga de abrir el archivo y ejecutar la función sysinfo_show, esta funcion proviene de seq_file.h


static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};
/*
Es una estructura definida en el kernel de Linux que contiene punteros a funciones que implementan las operaciones de archivo para una
 entrada en /proc. Estas funciones manejan operaciones como abrir, leer, escribir y cerrar la entrada en el sistema de archivos /proc.
*/

//La librería proc incluye las funciones proc_create y remove_proc_entry

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init); // se llama a la función sysinfo_init cuando se carga el módulo
module_exit(sysinfo_exit);