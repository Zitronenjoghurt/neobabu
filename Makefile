.PHONY: up down build logs migration entity

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

build:
	docker compose -f docker/docker-compose.yml build

logs:
	docker compose -f docker/docker-compose.yml logs -f

migration:
	cd ./core && sea-orm-cli migrate generate $(NAME)

entity:
	sea-orm-cli generate entity -o ./server/src/database/entity --database-url postgresql://admin:root@localhost:5432/neobabu