#!/bin/bash
set +x
export DENO_KV_ACCESS_TOKEN=$(openssl rand -base64 15)
echo "DENO_KV_ACCESS_TOKEN=$DENO_KV_ACCESS_TOKEN" | tee .env
docker run -it --init -p 4512:4512 -v ./data:/data ghcr.io/denoland/denokv --sqlite-path /data/denokv.sqlite serve --access-token $DENO_KV_ACCESS_TOKEN
