#!/usr/bin/sh
curl -i -N  \
    -H "Connection: Upgrade"  \
    -H "Upgrade: websocket"  \
    -H "Host: echo.websocket.org"  \
    -H "Origin: http://www.websocket.org"  \
    127.0.0.1:8080
