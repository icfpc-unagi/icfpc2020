.PHONY: all
all: data.pb.go

%.pb.go: %.proto
	mkdir -p .cache/pb
	protoc --proto_path=. --go_out=.cache/pb $*.proto
	cp .cache/pb/github.com/imos/icfpc2020/go/util/pb/$*.pb.go $*.pb.go
