.PHONY: *
.SILENT:

export DOCKER_BUILDKIT = 1
export DOCKER_COMPOSE_BUILDKIT = 1

run-server:
	@cargo run --bin server

run-client:
	@cargo run --bin client

docker/app/build:
	docker-compose -f .docker/docker-compose.yml build


docker/app/run:
	docker-compose -f .docker/docker-compose.yml up


docker/app/cli/shell:
	docker-compose -f .docker/docker-compose.yml exec client sh


docker/app/down:
	docker-compose -f .docker/docker-compose.yml down --volumes