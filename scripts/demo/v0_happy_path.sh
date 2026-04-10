#!/usr/bin/env bash
set -euo pipefail

payload='{"action_type":"issue_refund","target_id":"pay_123","amount":5000}'

echo "Creating execution..."
response="$(curl -sS -X POST http://127.0.0.1:3000/executions \
  -H 'content-type: application/json' \
  -H 'x-correlation-id: demo-v0-001' \
  -d "$payload")"

echo "$response"

execution_id="$(printf '%s' "$response" | python -c "import json,sys; print(json.load(sys.stdin)['execution']['id'])")"

echo
echo "Fetching execution ${execution_id}..."
curl -sS "http://127.0.0.1:3000/executions/${execution_id}" \
  -H 'x-correlation-id: demo-v0-fetch-001'
echo
