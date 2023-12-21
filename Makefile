tunnel:
	ssh -C -R 80:localhost:8000 serveo.net

ngrok:
	/tmp/ngrok http 8000

run:
	cargo shuttle run

.PHONY: tunnel
