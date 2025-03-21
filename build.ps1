$target_to_use = $env:BUILD_TARGET

$cross = $target_to_use.Contains("-cross")
$target = $($target_to_use.Replace("-cross", ""))
$root = "./leadcord"

try {
  rustup target add $target  
}
catch {
  "Continuing without installing $target"
}

if ($target.Contains("-musl")) {
  # CRT Static for MUSL targets
  $env:RUSTFLAGS = "-C target-feature=-crt-static"
}

if (!$cross) {
  "Using cargo"

  cargo build --release --target $target --manifest-path "$root/Cargo.toml"
}
else {
  "Using cross"

  cross build --release --target $target --manifest-path "$root/Cargo.toml" -Z build-std
}

Remove-Item -Recurse -Force build -ErrorAction SilentlyContinue
Remove-Item "$target.zip" -ErrorAction SilentlyContinue

New-Item build -ItemType Directory

if (Test-Path -Path docs) {
  Copy-Item -Path docs -Destination build -Recurse
}

Copy-Item -Path "$root/target/$target/release/*.dll*" -Destination "./build" -Recurse -ErrorAction SilentlyContinue
Copy-Item -Path "$root/target/$target/release/*.so*" -Destination "./build" -Recurse -ErrorAction SilentlyContinue
Copy-Item -Path "$root/target/$target/release/*.dylib*" -Destination "./build" -Recurse -ErrorAction SilentlyContinue

Compress-Archive -Path ./build/* -DestinationPath "$target.zip" -Verbose