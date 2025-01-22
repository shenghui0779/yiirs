#!/bin/bash
docker rm -f app_{{ name }}
docker rmi -f img_{{ name }}

docker build -t img_{{ name }} .
docker image prune -f

docker run -d --name=app_{{ name }} --restart=always --privileged -p 10085:50051 -p 10086:8000 -v /data/app_{{ name }}:/data img_{{ name }}
