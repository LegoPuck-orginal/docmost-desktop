Add-Type -AssemblyName System.Drawing

$assetsDir = "C:\Users\tim-m\IdeaProjects\docmost-desktop\assets"
if (-not (Test-Path $assetsDir)) {
    New-Item -ItemType Directory -Path $assetsDir -Force | Out-Null
}

function Render-DocmostImage([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $g.Clear([System.Drawing.Color]::Transparent)

    $scale = $size / 256.0

    # 1. Background Rounded Squircle
    $margin = 12.0 * $scale
    $rectSize = $size - 2 * $margin
    $radius = 48.0 * $scale

    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $path.AddArc($margin, $margin, $radius, $radius, 180, 90)
    $path.AddArc($margin + $rectSize - $radius, $margin, $radius, $radius, 270, 90)
    $path.AddArc($margin + $rectSize - $radius, $margin + $rectSize - $radius, $radius, $radius, 0, 90)
    $path.AddArc($margin, $margin + $rectSize - $radius, $radius, $radius, 90, 90)
    $path.CloseFigure()

    # Background Gradient
    $bgBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        (New-Object System.Drawing.PointF(0, 0)),
        (New-Object System.Drawing.PointF(0, $size)),
        [System.Drawing.Color]::FromArgb(255, 15, 18, 28),
        [System.Drawing.Color]::FromArgb(255, 9, 11, 18)
    )
    $g.FillPath($bgBrush, $path)

    # Subtle Border
    $borderPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(180, 55, 65, 95), [Math]::Max(1.0, 2.0 * $scale))
    $g.DrawPath($borderPen, $path)

    # 2. Docmost Stylized "D" & Document Emblem
    # Outer D loop
    $dPath = New-Object System.Drawing.Drawing2D.GraphicsPath
    $dx = 68.0 * $scale
    $dy = 52.0 * $scale
    $dw = 120.0 * $scale
    $dh = 152.0 * $scale
    $thick = 32.0 * $scale

    # Main D shape
    $dPath.AddRectangle((New-Object System.Drawing.RectangleF($dx, $dy, $thick, $dh)))
    
    # Rounded bowl of D
    $bowlX = $dx + 16.0 * $scale
    $bowlY = $dy
    $bowlW = $dw - 16.0 * $scale
    $bowlH = $dh
    $dPath.AddArc($bowlX, $bowlY, $bowlW, $bowlH, 270, 180)
    $dPath.CloseFigure()

    # Inner cutout of D
    $cutoutPath = New-Object System.Drawing.Drawing2D.GraphicsPath
    $inX = $dx + $thick
    $inY = $dy + 28.0 * $scale
    $inW = ($dw - $thick) * 0.72
    $inH = $dh - 56.0 * $scale
    $cutoutPath.AddArc($inX - 4.0 * $scale, $inY, $inW, $inH, 270, 180)
    $cutoutPath.CloseFigure()

    # Subtract cutout
    $region = New-Object System.Drawing.Region($dPath)
    $region.Exclude($cutoutPath)

    # Vibrant Gradient Brush for "D" (Indigo to Purple to Pink)
    [float]$p1x = [float]$dx
    [float]$p1y = [float]$dy
    [float]$p2x = [float]$dx + [float]$dw
    [float]$p2y = [float]$dy + [float]$dh
    $p1 = New-Object System.Drawing.PointF($p1x, $p1y)
    $p2 = New-Object System.Drawing.PointF($p2x, $p2y)
    $emblemBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $p1,
        $p2,
        [System.Drawing.Color]::FromArgb(255, 99, 102, 241),
        [System.Drawing.Color]::FromArgb(255, 192, 38, 211)
    )
    $g.FillRegion($emblemBrush, $region)

    # Document folded-corner accent dot in top right
    $dotSize = 14.0 * $scale
    $dotX = $dx + $dw - $dotSize - 4.0 * $scale
    $dotY = $dy + 12.0 * $scale
    $dotBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 129, 140, 248))
    $g.FillEllipse($dotBrush, $dotX, $dotY, $dotSize, $dotSize)

    $g.Dispose()
    return $bmp
}

# Generate PNGs and collect memory streams
$sizes = @(256, 128, 64, 48, 32, 16)
$pngBytesList = @()

foreach ($s in $sizes) {
    $bmp = Render-DocmostImage $s
    if ($s -eq 256) {
        $bmp.Save("$assetsDir\icon.png", [System.Drawing.Imaging.ImageFormat]::Png)
    }
    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $pngBytesList += ,@($s, $ms.ToArray())
    $ms.Dispose()
    $bmp.Dispose()
}

# Build Windows ICO File
$icoPath = "$assetsDir\icon.ico"
$fs = [System.IO.File]::Create($icoPath)
$bw = New-Object System.IO.BinaryWriter($fs)

$count = $sizes.Count
# ICONDIR
$bw.Write([uint16]0)     # Reserved
$bw.Write([uint16]1)     # Type = 1 (Icon)
$bw.Write([uint16]$count) # Image count

# Calculate offsets
# Header size = 6 bytes + (count * 16 bytes)
$offset = 6 + ($count * 16)

# Write ICONDIRENTRY entries
foreach ($entry in $pngBytesList) {
    $s = $entry[0]
    $data = $entry[1]

    $w = if ($s -ge 256) { 0 } else { [byte]$s }
    $h = if ($s -ge 256) { 0 } else { [byte]$s }

    $bw.Write([byte]$w)           # Width
    $bw.Write([byte]$h)           # Height
    $bw.Write([byte]0)            # Color count
    $bw.Write([byte]0)            # Reserved
    $bw.Write([uint16]1)          # Planes
    $bw.Write([uint16]32)         # Bit count
    $bw.Write([uint32]$data.Length) # Bytes in resource
    $bw.Write([uint32]$offset)    # Image offset

    $offset += $data.Length
}

# Write PNG payloads
foreach ($entry in $pngBytesList) {
    $data = $entry[1]
    $bw.Write($data)
}

$bw.Flush()
$bw.Dispose()
$fs.Dispose()

# Generate raw 32x32 RGBA byte array for Tao runtime window icon
$bmp32 = Render-DocmostImage 32
$bytes32 = New-Object byte[] (32 * 32 * 4)
$i = 0
for ($y = 0; $y -lt 32; $y++) {
    for ($x = 0; $x -lt 32; $x++) {
        $c = $bmp32.GetPixel($x, $y)
        $bytes32[$i++] = $c.R
        $bytes32[$i++] = $c.G
        $bytes32[$i++] = $c.B
        $bytes32[$i++] = $c.A
    }
}
$bmp32.Dispose()

$srcDir = "C:\Users\tim-m\IdeaProjects\docmost-desktop\src"
$byteStrings = ($bytes32 | ForEach-Object { "0x{0:x2}" -f $_ }) -join ", "
$iconRsContent = "pub const ICON_RGBA_32: &[u8] = &[$byteStrings];`n"
Set-Content -Path "$srcDir\icon_data.rs" -Value $iconRsContent -NoNewline

Write-Host "Erfolgreich erstellt:" -ForegroundColor Green
Write-Host " - $assetsDir\icon.png"
Write-Host " - $assetsDir\icon.ico"
Write-Host " - $srcDir\icon_data.rs"
