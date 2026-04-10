$headers = @{
    "content-type" = "application/json"
    "x-correlation-id" = "demo-v0-001"
}

$body = @{
    action_type = "issue_refund"
    target_id = "pay_123"
    amount = 5000
} | ConvertTo-Json

Write-Host "Creating execution..."
$response = Invoke-RestMethod -Method Post -Uri "http://127.0.0.1:3000/executions" -Headers $headers -Body $body
$response | ConvertTo-Json -Depth 6

$executionId = $response.execution.id

Write-Host ""
Write-Host "Fetching execution $executionId..."
$fetchHeaders = @{ "x-correlation-id" = "demo-v0-fetch-001" }
Invoke-RestMethod -Method Get -Uri "http://127.0.0.1:3000/executions/$executionId" -Headers $fetchHeaders | ConvertTo-Json -Depth 6
