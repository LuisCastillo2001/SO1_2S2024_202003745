#!/bin/bash
imagenes=('alto_1' 'alto_2' 'bajo_1' 'bajo_2')

for i in {1..10}; do
    random=$(($RANDOM%4))
    nombre_contenedor=$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 12)
    docker run -d --name $nombre_contenedor ${imagenes[$random]}
done