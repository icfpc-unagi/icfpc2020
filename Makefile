.PHONY: bootstrap
bootstrap:
	cd docker && make bootstrap

.PHONY: docker
docker:
	cd docker && make docker

.PHONY: launcher
launcher: docker/launcher

.PHONY: upload-launcher
upload-launcher: docker/upload-launcher

.PHONY: push-docker
push-docker: docker/push-docker-latest

.PHONY: docker/%
docker/%: docker
	docker run -v $(shell pwd):/work -w /work \
		-v /var/run/docker.sock:/var/run/docker.sock \
		imos/icfpc2020 make "orig@$*"

################################################################################
# Targets run inside unagi image.
################################################################################

.PHONY: orig@launcher
orig@launcher:
	cd go/cmd/launcher && make -j 6
	cp script/launcher.sh build/launcher
	chmod +x build/launcher

.PHONY: orig@upload-launcher
orig@upload-launcher:
	cd go/cmd/launcher && make -j 6 upload

.PHONY: orig@upload-installer
orig@upload-installer:
	gsutil cp script/install-launcher.sh gs://icfpc-public-data/install.sh

.PHONY: orig@orig-upload
orig@upload: orig@upload-launcher orig@upload-installer

.PHONY: orig@push-docker-%
orig@push-docker-%:
	bash script/push-docker-image.sh "$*"
