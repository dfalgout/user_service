.PHONY: watch

watch:
	docker-compose up -d
	systemfd --no-pid -s http::8088 -- cargo watch -x run