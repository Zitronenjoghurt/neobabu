.PHONY: up down build logs db-up db-down migration entity

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

build:
	docker compose -f docker/docker-compose.yml build

logs:
	docker compose -f docker/docker-compose.yml logs -f

db-up:
	docker compose -f docker/docker-compose.db.yml up -d

db-down:
	docker compose -f docker/docker-compose.db.yml down

migration:
	cd ./core && sea-orm-cli migrate generate $(NAME)

entity:
	sea-orm-cli generate entity -o ./core/src/database/entity --database-url postgresql://admin:root@localhost:54323/neobabu