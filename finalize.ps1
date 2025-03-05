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

$metadata = @{
  package     = $package
  version     = $version
  authors     = $authors
  description = $description
  keywords    = $keywords
  platforms   = @()
  type        = "dylib"
}

New-Item dist -ItemType Directory -ErrorAction SilentlyContinue

Copy-Item -Path "./leadcord/src" -Destination "./dist/src/src" -Recurse -Force
Copy-Item -Path ./leadcord/* -Include *.toml -Destination "./dist/src/" -Force

foreach ($target in $toolchains) {
  try {
    Expand-Archive -Path "$target.zip" -DestinationPath "./dist/lib/$target" -Force 

    $metadata.platforms += $target
  }
  catch {
    Remove-Item "./dist/lib/$target" -Recurse -Force
  }
}

ConvertTo-Json $metadata | Out-File "./dist/pkgcache"

Compress-Archive -Path "./dist/*" -DestinationPath "leadpkg.zip" -Force