FROM imos/icfpc2020:data AS data

FROM nginx:alpine

COPY --from=data /data/domain-fullchain.pem /data/domain-fullchain.pem
COPY --from=data /data/domain-privkey.pem /data/domain-privkey.pem
RUN chmod 600 /data/*
COPY htpasswd /etc/nginx/htpasswd
COPY default.conf /etc/nginx/conf.d/
COPY index.html /usr/share/nginx/html/
