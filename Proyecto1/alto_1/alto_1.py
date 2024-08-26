import time
import random

arreglo_1 = []
while True: 
    
    arreglo_2 = ["hola mundo" * 1000 for _ in range(9999)]  

   
    arreglo_1 = [arreglo_2[:] for _ in range(80)]  

    
    time.sleep(0.70)
