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

.PHONY: data
data: ../build/data
	docker build --file data.Dockerfile \
		-t imos/icfpc2020:data ../build/data

###############################################################################
# Main image
###############################################################################

.PHONY: docker
docker: bootstrap data Dockerfile ../build/docker
	docker build --file Dockerfile -t imos/icfpc2020 ../build/docker

.PHONY: sql
sql: data sql.Dockerfile
	docker build --file sql.Dockerfile -t imos/icfpc2020:sql .

.PHONY: phpmyadmin
phpmyadmin: data phpmyadmin.Dockerfile
	docker build --file phpmyadmin.Dockerfile \
		--build-arg UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
		-t imos/icfpc2020:phpmyadmin .

.PHOY: nginx
nginx: data nginx/Dockerfile
	docker build --file nginx/Dockerfile \
		-t imos/icfpc2020:nginx ./nginx

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
encrypt@%: secret/% bootstrap
	docker run --rm -i imos/icfpc2020:bootstrap encrypt \
		< secret/$* > data/$*.encrypted

.PHONY: data@%.encrypted
data@%.encrypted:
	docker run --rm -i imos/icfpc2020:bootstrap decrypt \
		< data/$*.encrypted > ../build/data/$*

.PHONY: data@%
data@%:
	cp data/$* ../build/data/$*

.PHONY: ../build/data
../build/data:
	-rm -rf ../build/data
	mkdir -p ../build/data
	for f in data/*; do make "data@$${f#data/}"; done
