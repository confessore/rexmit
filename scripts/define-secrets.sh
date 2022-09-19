#!/bin/sh

read -p "please enter the postgres user : " postgresUser
read -p "please enter the postgres password : " postgresPassword
read -p "please enter the discord token : " discordToken

if [ "$(dirname $0)" = "." ]
then
    mkdir -p ../secrets
    cd ../secrets
else
    parent=$(dirname $(dirname "$0"))
    mkdir -p "$parent"/secrets
    cd "$parent"/secrets
fi

echo "generating secrets..."
echo "$postgresUser" > postgres-user
echo "$postgresPassword" > postgres-password
echo "$discordToken" > discord-token
echo "secrets generated!"
