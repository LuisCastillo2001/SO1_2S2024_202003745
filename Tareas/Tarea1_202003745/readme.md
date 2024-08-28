# Codigo realizado

```c
#include <linux/module.h>
#include <linux/kernel.h> //trae todas las funciones que se pueden usar en el kernel
#include <linux/init.h>
#include <linux/proc_fs.h> // trae las funciones para crear archivos en /proc
#include <linux/seq_file.h> // trae las funciones para escribir en archivos en /proc
#include <linux/mm.h> // trae las funciones para manejar la memoria
#include <linux/sched.h> // trae las funciones para manejar los procesos
#include <linux/timer.h> // trae las funciones para manejar los timers
#include <linux/jiffies.h> // trae las funciones para manejar los jiffies, que son los ticks del sistema

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Luis Castillo");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y CPU");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo" // nombre del archivo en /proc, y así es como se crea la macro


static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si; // estructura que contiene la informacion de la memoria
    struct task_struct *task; //contiene toda la información relevante de un proceso
    si_meminfo(&si); // obtiene la informacion de la memoria, y con la función si_meminfo se obtiene la información de la memoria
    int num_parent_processes = 0;
    int num_child_processes = 0;
    
    /*  
        El seq_printf se encarga de escribir en el archivo en /proc
        - m: es el archivo en /pro
    */

    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * 4);
    seq_printf(m, "Free RAM: %lu KB\n", si.freeram * 4);
    seq_printf(m, "Shared RAM: %lu KB\n", si.sharedram * 4);
    seq_printf(m, "Buffer RAM: %lu KB\n", si.bufferram * 4);
    seq_printf(m, "Total Swap: %lu KB\n", si.totalswap * 4);
    seq_printf(m, "Free Swap: %lu KB\n", si.freeswap * 4);

    //seq_printf(m, "Number of processes: %d\n", num_online_cpus());

   //Se creo un puntero para task, esto quiere decir que al momento de hacer esto
   //&task se esta obteniendo un puntero a la dirección de memoria de task, y con la "->" se accede a los elementos de la estructura
    for_each_process(task){
        if (!list_empty(&task->children)){
            num_parent_processes++;
        }else{
            num_child_processes++;
        }
        
    }
    seq_printf(m, "Number of processes: %d\n", num_parent_processes);
    seq_printf(m, "Number of child processes: %d\n", num_child_processes);

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
module_exit(sysinfo_exit); ```
