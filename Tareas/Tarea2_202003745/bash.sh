#!/bin/bash
for i in {1..10}; do
    nombre=$(uuidgen | cut -c 1-8)
    sudo docker run -d --name $nombre alpine
done
