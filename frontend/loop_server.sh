#!/bin/sh

while true
do
  if ! pgrep -x "frontend-server" > /dev/null
  then
    stack exec -- frontend-server
  fi
done
