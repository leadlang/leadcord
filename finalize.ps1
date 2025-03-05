$toolchains = Get-Content "./toolchains.txt"

$cargo = (Get-Content "./leadcord/Cargo.toml") -join "
"

$cargo = ConvertFrom-Toml $cargo

$package = $cargo.lib.name
$version = $cargo.package.version
$authors = $cargo.package.authors
$description = $cargo.package.description
$keywords = $cargo.package.keywords

if ($null -eq $authors) {
  $authors = @()
}

if ($null -eq $keywords) {
  $keywords = @()
}

if ($null -eq $description) {
  $description = ""
}

$metadata = @{
  package     = $package
  version     = $version
  authors     = $authors
  description = $description
  keywords    = $keywords
  uses_new    = $true
  platforms   = @()
  type        = "dylib"
}

New-Item dist -ItemType Directory -ErrorAction SilentlyContinue

Copy-Item -Path "./leadcord/src" -Destination "./dist/src/src" -Recurse -Force
Copy-Item -Path ./leadcord/* -Include *.toml -Destination "./dist/src/" -Force

New-Item "./dist/lib" -ItemType Directory -ErrorAction Suspend

foreach ($target in $toolchains) {
  if (Test-Path -Path "./$target.zip") {
    $metadata.platforms += $target
  }
}

ConvertTo-Json $metadata | Out-File "./dist/pkgcache"

Compress-Archive -Path "./dist/*" -DestinationPath "leadpkg.zip" -Force