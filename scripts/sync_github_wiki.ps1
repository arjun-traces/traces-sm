# Script to synchronize docs/ folder to GitHub Wiki repo (arjun-traces/traces-sm.wiki.git)
param (
    [string]$WikiRepoUrl = "https://github.com/arjun-traces/traces-sm.wiki.git"
)

$wikiDir = "$env:TEMP\traces-sm-wiki"
if (Test-Path $wikiDir) { Remove-Item -Path $wikiDir -Recurse -Force }

Write-Host "Cloning GitHub Wiki repository..." -ForegroundColor Cyan
git clone $WikiRepoUrl $wikiDir

Write-Host "Copying documentation pages to Wiki format..." -ForegroundColor Yellow
Copy-Item -Path "docs/TECHNICAL_SPECIFICATION.md" -Destination "$wikiDir\Technical-Specification.md" -Force
Copy-Item -Path "docs/PRODUCT_SPECIFICATION.md" -Destination "$wikiDir\Product-Specification.md" -Force
Copy-Item -Path "docs/CONFORMANCE_REPORT.md" -Destination "$wikiDir\Conformance-Report.md" -Force
Copy-Item -Path "docs/ARCHITECTURE_WALKTHROUGH.md" -Destination "$wikiDir\Architecture-Walkthrough.md" -Force
Copy-Item -Path "docs/WINDOWS_BUILD_GUIDE.md" -Destination "$wikiDir\Windows-Build-Guide.md" -Force
Copy-Item -Path "docs/PAGE_BY_PAGE_DESIGN.md" -Destination "$wikiDir\Page-by-Page-Design.md" -Force
Copy-Item -Path "docs/KNOWLEDGE_BANK.md" -Destination "$wikiDir\Knowledge-Bank.md" -Force

Set-Location -Path $wikiDir
git add .
git commit -m "Sync documentation from main repo docs/ folder"
git push origin master

Write-Host "GitHub Wiki synchronized successfully!" -ForegroundColor Green
