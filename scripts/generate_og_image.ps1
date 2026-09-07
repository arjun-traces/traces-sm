Add-Type -AssemblyName System.Drawing

$width = 1200
$height = 630

$bmp = New-Object System.Drawing.Bitmap($width, $height)
$g = [System.Drawing.Graphics]::FromImage($bmp)

$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

# Background fill (#030712)
$bgColor = [System.Drawing.ColorTranslator]::FromHtml("#030712")
$bgBrush = New-Object System.Drawing.SolidBrush($bgColor)
$g.FillRectangle($bgBrush, 0, 0, $width, $height)

# Subtly draw border container (#1e293b)
$pen = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#1e293b"), 4)
$g.DrawRectangle($pen, 40, 40, $width - 80, $height - 80)

# Badge container fill (#1e1b4b) & border (#6366f1)
$badgeBg = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#1e1b4b"))
$badgePen = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#6366f1"), 2)
$g.FillRectangle($badgeBg, 80, 80, 440, 50)
$g.DrawRectangle($badgePen, 80, 80, 440, 50)

# Badge text
$badgeFont = New-Object System.Drawing.Font("Consolas", 14, [System.Drawing.FontStyle]::Bold)
$badgeBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#a5b4fc"))
$g.DrawString("SGX Enclave Hardware Security", $badgeFont, $badgeBrush, 95, 93)

# Title: traces-sm
$titleFont = New-Object System.Drawing.Font("Segoe UI", 48, [System.Drawing.FontStyle]::Bold)
$titleBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#ffffff"))
$g.DrawString("traces-sm", $titleFont, $titleBrush, 80, 160)

# Descriptor: Rust-native SGX secrets and key management framework
$descFont = New-Object System.Drawing.Font("Segoe UI", 24, [System.Drawing.FontStyle]::Bold)
$descBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#818cf8"))
$g.DrawString("Rust-native SGX secrets and key management framework", $descFont, $descBrush, 80, 245)

# One-sentence definition verbatim
$bodyFont = New-Object System.Drawing.Font("Segoe UI", 16, [System.Drawing.FontStyle]::Regular)
$bodyBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#94a3b8"))
$bodyText = "traces-sm is an open-source key and secret management framework that runs its cryptographic operations inside an Intel SGX enclave, written entirely in Rust on Fortanix EDP."
$rect = New-Object System.Drawing.RectangleF(80, 320, 1040, 120)
$g.DrawString($bodyText, $bodyFont, $bodyBrush, $rect)

# Footer pills / tags
$pillFont = New-Object System.Drawing.Font("Consolas", 12, [System.Drawing.FontStyle]::Bold)
$pillBg = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#0f172a"))
$pillBorder = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#334155"), 2)
$pillTextBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#cbd5e1"))

$pills = @("x86_64-fortanix-unknown-sgx", "NIST SP 800-57 Aligned", "Zeroize RAM Wiping", "Apache-2.0 License")
$xPos = 80
foreach ($pill in $pills) {
    $size = $g.MeasureString($pill, $pillFont)
    $pWidth = [int]$size.Width + 24
    $g.FillRectangle($pillBg, $xPos, 490, $pWidth, 42)
    $g.DrawRectangle($pillBorder, $xPos, 490, $pWidth, 42)
    $g.DrawString($pill, $pillFont, $pillTextBrush, ($xPos + 12), 502)
    $xPos += $pWidth + 20
}

if (-not (Test-Path "docs/images")) {
    New-Item -ItemType Directory -Path "docs/images" | Out-Null
}

$bmp.Save("docs/images/og-image.png", [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose()
$bmp.Dispose()
Write-Host "Successfully generated docs/images/og-image.png"
