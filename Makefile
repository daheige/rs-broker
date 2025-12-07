dev_image_name :=rs-dev
version :=v1.0
image_name :=rs-broker-svc
service_name :=rs-broker
root_dir :=$(shell pwd)

build-dev:
	docker build . -f Dockerfile-dev -t ${dev_image_name}:${version}
kafka:
	docker run -itd -p 9092:9092 --name kafka-dev apache/kafka-native:4.1.1
build:
	docker build . -f Dockerfile -t ${image_name}:${version}
run:
	docker run -itd --name=${service_name} -p 50051:50051 -p 8090:8090 \
	-v ${root_dir}/.env:/app/.env -itd ${image_name}:${version}
exec:
	docker exec -it ${service_name} /bin/bash