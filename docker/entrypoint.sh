#!/bin/bash

cp /etopa/etopa.conf /etc/etopa.conf &> /dev/null
cp /etopa/config.js /var/www/etopa/config.js &> /dev/null
cp /etopa/nginx.conf /etc/nginx/nginx.conf &> /dev/null
cp /etopa/cert.pem /etc/nginx/cert.pem &> /dev/null
cp /etopa/privkey.pem /etc/nginx/privkey.pem &> /dev/null
cp /etopa/fullchain.pem /etc/nginx/fullchain.pem &> /dev/null
cp /etopa/dhparam.pem /etc/nginx/dhparam.pem &> /dev/null

cp /etc/etopa.conf /etopa/etopa.conf &> /dev/null
cp /var/www/etopa/config.js /etopa/config.js &> /dev/null
cp /etc/nginx/nginx.conf /etopa/nginx.conf &> /dev/null
cp /etc/nginx/cert.pem /etopa/cert.pem &> /dev/null
cp /etc/nginx/privkey.pem /etopa/privkey.pem &> /dev/null
cp /etc/nginx/fullchain.pem /etopa/fullchain.pem &> /dev/null
cp /etc/nginx/dhparam.pem /etopa/dhparam.pem &> /dev/null

chown nginx:nginx /var/www/etopa/config.js
chown nginx:nginx /etc/nginx/nginx.conf
chown nginx:nginx /etc/nginx/*.pem

nginx
etopa
