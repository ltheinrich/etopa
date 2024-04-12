FROM alpine:latest

RUN apk add --no-cache nginx bash
ADD docker/cert.pem /etc/nginx/cert.pem
ADD docker/privkey.pem /etc/nginx/privkey.pem
ADD docker/cert.pem /etc/nginx/fullchain.pem
ADD docker/dhparam.pem /etc/nginx/dhparam.pem
ADD docker/nginx.conf /etc/nginx/nginx.conf

RUN mkdir /var/www/etopa
ADD target/build/etopa.tar.xz /var/www/etopa/
RUN chown -R nginx:nginx /var/www/etopa

ADD target/build/etopa /usr/local/bin/etopa
RUN chmod +x /usr/local/bin/etopa

ADD docker/entrypoint.sh /root/entrypoint.sh
RUN chmod +x /root/entrypoint.sh

VOLUME ["/etopa"]
WORKDIR /etopa
ENTRYPOINT  ["bash", "/root/entrypoint.sh"]
