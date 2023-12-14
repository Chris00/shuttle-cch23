HOST = christophe@mathinfo.umons.ac.be
PORT = 2222
PASS = $(shell grep "# pass:" $(HOME)/.ssh/config | sed 's/# pass: \(.*\)/\1/')
SSHPASS = sshpass -p "$(PASS)"

BIND_ADDRESS=$(shell host math.umons.ac.be | sed -e 's/.*address *//')

# Require to enable "GatewayPorts" to "yes" on $(HOST) (sshd_config)
tunnel:
	ssh -R 80:localhost:8000 serveo.net
# Port forwarding to UMONS fails, probably because of the firewall
#	@echo "Establish a tunnel port 8000 → $(BIND_ADDRESS):$(PORT)"
#	$(SSHPASS) ssh -v -R $(BIND_ADDRESS):$(PORT):localhost:8000 $(HOST)

run:
	cargo shuttle run
