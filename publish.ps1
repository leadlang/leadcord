$v = Get-Content .version

Write-Output PowerShell

Write-Output "TAG_NAME=v$v" >> "$env:GITHUB_OUTPUT"