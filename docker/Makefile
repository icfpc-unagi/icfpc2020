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

.PHONY: data/%.encrypted
data/%.encrypted: secret/% bootstrap
	docker run --rm -i -v $(pwd):/work imos/icfpc2020:bootstrap encrypt \
		< secret/$* > data/$*.encrypted
