#!/bin/bash

SCRIPTNAME=$(readlink -f $0)

rm -rf /var/www/html/*

./coverage-create.sh

if [[ $? != 0 ]]; then
    printf "%s: could not create coverage\n" $SCRIPTNAME
    exit -1
fi

cp -r /app/output/* /var/www/html

nginx -g "daemon off; master_process off;"
