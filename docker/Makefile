usage:
	@echo 'Usage: make (upload|build)'
.PHONY: usage

###############################################################################
# Bootstrap image
###############################################################################
.PHONY: ../build/bootstrap
../build/bootstrap:
	-rm -rf ../build/bootstrap
	mkdir -p ../build/bootstrap
	cp -a ../bin ../build/bootstrap/bin

.PHONY: bootstrap
bootstrap: bootstrap.Dockerfile ../build/bootstrap
	docker build --file bootstrap.Dockerfile \
		--build-arg UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
		-t imos/icfpc2020:bootstrap ../build/bootstrap

###############################################################################
# Main image
###############################################################################

.PHONY: docker
docker: bootstrap Dockerfile ../build/docker
	docker build --file Dockerfile -t imos/icfpc2020 ../build/docker

###############################################################################
# Utility targets
###############################################################################

# Encrypt a secret and save it to data.
#
# Usage:
#   1. Place a file under //docker/secret (e.g., //docker/secret/foo).
#   2. Run `make encrypt@foo` in //docker directory, and then
#      //docker/data/foo.encrypted should be generated.
.PHONY: encrypt@%.encrypted
encrypt@%.encrypted: secret/% bootstrap
	docker run --rm -i imos/icfpc2020:bootstrap encrypt \
		< secret/$* > data/$*.encrypted

.PHONY: data@%.encrypted
data@%.encrypted:
	docker run --rm -i imos/icfpc2020:bootstrap decrypt \
		< data/$*.encrypted > ../build/docker/$*

.PHONY: data@%
data@%:
	cp data/$* ../build/docker/$*

.PHONY: ../build/docker
../build/docker:
	-rm -rf ../build/docker
	mkdir -p ../build/docker
	for f in data/*; do make "data@$${f#data/}"; done
