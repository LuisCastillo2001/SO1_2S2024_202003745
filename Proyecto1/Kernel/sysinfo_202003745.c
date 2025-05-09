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

#define PROC_NAME "sysinfo_202003745" 
#define MAX_CMDLINE_LENGTH 4096
#define CONTAINER_ID_LENGTH 12
static char *get_container_id(const char *cmdline) {
    char *id_start = strstr(cmdline, "-id "); 
    if (id_start) {
        id_start += 4; 
        
        
        char *container_id = kmalloc(CONTAINER_ID_LENGTH + 1, GFP_KERNEL);
        if (container_id) {
            strncpy(container_id, id_start, CONTAINER_ID_LENGTH);
            container_id[CONTAINER_ID_LENGTH] = '\0';
            return container_id;
        }
    }
    return NULL;
}



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
    

    for_each_process(task) {
        char *cmdline = get_process_cmdline(task);
        
        if (strcmp(task->comm, "containerd-shim") == 0) {
           
            
            if (cmdline) {
                char *container_id = get_container_id(cmdline);
                
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

            
            if (task->mm) {
               
                vsz = task->mm->total_vm << (PAGE_SHIFT - 10);
              
                rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);
                
                mem_usage = (rss * 10000) / totalram;
            }

           
            unsigned long total_time = task->utime + task->stime;
            cpu_usage = (total_time * 10000) / total_jiffies;
            cmdline = get_process_cmdline(task);

            
           

           
            
            seq_printf(m, "     \"Cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "     \"MemoryUsage\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "     \"CPUUsage\": %lu.%02lu,\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "     \"VSZ\": %lu,\n", vsz);
            seq_printf(m, "     \"RSS\": %lu\n", rss);
            seq_printf(m, "     }\n");


           
            if (cmdline) {
                kfree(cmdline);
            }
        }
    }

    
    seq_printf(m, "\n  ]\n}\n");

    return 0;
}


 



static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}



static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};
/

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init); 
module_exit(sysinfo_exit);