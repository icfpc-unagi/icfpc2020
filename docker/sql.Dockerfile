FROM imos/icfpc2020:data AS data

FROM gcr.io/cloudsql-docker/gce-proxy:1.16

COPY --from=data /data/service_account.json /config/service_account.json

CMD ["/cloud_sql_proxy", \
     "-instances=icfpc-primary:asia-northeast1:tokyo=tcp:0.0.0.0:3306", \
     "-credential_file=/config/service_account.json"]
