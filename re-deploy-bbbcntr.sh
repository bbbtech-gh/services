#!/usr/bin/bash 
# update.sh

cd bbbcntr

git pull

cd ..

BBBCNTR_BUILD_VERSION=$(git rev-parse HEAD)

echo "$(date -u +"[%d/%m/%Y %T]"): Releasing new BBBCNTR version $(BBBCNTR_BUILD_VERSION) 
$(date -u +"[%d/%m/%Y %T]"): Running build...
"

docker compose rm -f
docker compose build

OLD_CONTAINER=$(docker ps -aqf "name=bbbcntr")
echo "$(date -u +"[%d/%m/%Y %T]"): Scaling server up..."

BUILD_VERSION=$BBBCNTR_BUILD_VERSION docker compose up -d --no-deps --scale bbbcntr=2 --no-recreate bbbcntr

sleep 20

echo "$(date -u +"[%d/%m/%Y %T]"): Scaling down old server..."
docker container rm -f $OLD_CONTAINER
docker compose -d --no-deps --scale bbbcntr=1 --no-recreate bbbcntr

# echo "$(date -u +"[%d/%m/%Y %T]"): moving out of bbbcntr directory..."
# cd ..

echo "$(date -u +"[%d/%m/%Y %T]"): Reloading Caddy..."
CADDY_CONTAINER=$(docker ps -aqf "name=caddy")
docker exec $CADDY_CONTAINER caddy reload -c ./Caddyfile