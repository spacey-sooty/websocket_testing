#!/usr/bin/sh
curl -i -N  \
    -H "Connection: Upgrade"  \
    -H "Upgrade: websocket"  \
    -H "Host: 127.0.0.1:8080"  \
    -H "Origin: 127.0.0.1:8080"  \
   127.0.0.1:8080

