run:
	cargo run

run.migrate.refresh:
	sea-orm-cli migrate refresh -d portal_migration

run.migrate.fresh:
	sea-orm-cli migrate fresh -d portal_migration

run.entity.gen:
	sea-orm-cli generate entity -o portal_schema/src --lib

docker.compose:
	docker compose up --build -d --remove-orphans

docker.start:
	docker compose start

docker.stop:
	docker compose stop
