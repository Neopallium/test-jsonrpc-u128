#!/bin/sh
#

curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"jsonrpc": "2.0", "method": "get_auth", "params": [], "id": 1}' \
	http://localhost:3030/
