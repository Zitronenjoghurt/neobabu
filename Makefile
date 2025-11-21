.PHONY: up down build logs db-up db-down bot-up bot-down bot-build bot-logs web-up web-down web-build web-logs web-dev migration entity

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

build:
	docker image prune -f
	docker compose -f docker/docker-compose.yml build

logs:
	docker compose -f docker/docker-compose.yml logs -f

db-up:
	docker compose -f docker/docker-compose.db.yml up -d

db-down:
	docker compose -f docker/docker-compose.db.yml down

migration:
	cd ./core && sea-orm-cli migrate generate $(NAME)

bot-up:
	docker compose -f docker/docker-compose.bot.yml up -d

bot-down:
	docker compose -f docker/docker-compose.bot.yml down

bot-build:
	docker image prune -f
	docker compose -f docker/docker-compose.bot.yml build

bot-logs:
	docker compose -f docker/docker-compose.bot.yml logs -f

web-up:
	docker compose -f docker/docker-compose.web.yml up -d

web-down:
	docker compose -f docker/docker-compose.web.yml down

web-build:
	docker image prune -f
	docker compose -f docker/docker-compose.web.yml build

web-logs:
	docker compose -f docker/docker-compose.web.yml logs -f

web-dev:
	cd web/dashboard && npm run dev

entity:
	sea-orm-cli generate entity -o ./core/src/database/entity --database-url postgresql://admin:root@localhost:54323/neobabu