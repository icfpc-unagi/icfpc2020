.PHONY: install
install:
	mkdir -p /opt/nginx
	cp docker-compose.yml /opt/nginx/docker-compose.yml
	cp nginx.service /etc/systemd/system/nginx.service
	systemctl daemon-reload
	systemctl restart nginx
