#!/bin/bash

./coverage-create.sh

if [[ $? != 0 ]]; then
    exit -1
fi

cp -r /app/output/* /var/www/html

nginx -g "daemon off; master_process off;"
