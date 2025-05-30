#!/bin/bash

rm -rf /var/www/html/*

./coverage-create.sh

if [[ $? != 0 ]]; then
    printf "could not create coverage\n"
    exit -1
fi

cp -r /app/output/* /var/www/html

nginx -g "daemon off; master_process off;"
