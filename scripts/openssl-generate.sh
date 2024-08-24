#!/bin/sh

if [ "$(dirname $0)" = "." ]
then
    mkdir -p ../etc/openssl
    cd ../etc/openssl
else
    parent=$(dirname $(dirname "$0"))
    mkdir -p "$parent"/etc/openssl
    cd "$parent"/etc/openssl
fi
openssl req -x509 -nodes -days 365 -newkey rsa:4096 -keyout localhost.key -out localhost.crt -config localhost.conf -passin pass:root
openssl pkcs12 -export -out localhost.pfx -inkey localhost.key -in localhost.crt -passout pass:root