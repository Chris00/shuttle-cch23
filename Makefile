tunnel:
	ssh -R 80:localhost:8000 ssi.sh
#	ssh -R 80:localhost:8000 nokey@localhost.run

serveo:
	ssh -R 80:localhost:8000 serveo.net

ngrok:
	/tmp/ngrok http 8000

run:
	cargo shuttle run

.PHONY: tunnel serveo
