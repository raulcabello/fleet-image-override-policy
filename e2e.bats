#!/usr/bin/env bats

@test "Accept a valid name" {
	run kwctl run  --request-path test_data/deployment_creation.json  annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
	[ $(expr "$output" : '.*"allowed":true.*') -ne 0 ]
 }
