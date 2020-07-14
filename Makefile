.PHONY: bootstrap
bootstrap:
	cd docker && make bootstrap

.PHONY: docker
docker:
	cd docker && make docker
