#!/bin/sh
SERVICE="nodemon ap1p"
if ps ax | grep -i "$SERVICE" | grep -v grep >/dev/null
then
    echo "$SERVICE is running"
else
    echo "$SERVICE stopped"
    # uncomment to start nginx if stopped
    # systemctl start nginx
    # mail
fi
