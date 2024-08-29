#!/bin/bash

# Ruta del script que deseas ejecutar
SCRIPT="/home/cluiis/Documentos/SO1_2S2024_202003745/Proyecto1/Prueba_contenedores/prueba.sh"

# Bucle infinito
while true; do
    # Ejecuta el script en segundo plano
    bash "$SCRIPT" 
    
    # Espera 30 segundos antes de volver a ejecutar
    sleep 30
done

