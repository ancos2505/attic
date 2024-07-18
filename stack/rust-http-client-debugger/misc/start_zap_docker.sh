#!/bin/sh
docker run --rm --name zap-headless -p 8080:8080 -p 8090:8090 -v $HOME/zap-data/:/zap/wrk/:rw -t ghcr.io/zaproxy/zaproxy:stable zap-webswing.sh -daemon -host 0.0.0.0 -port 8080 -config api.addrs.addr.name=.* -config api.addrs.addr.regex=true

