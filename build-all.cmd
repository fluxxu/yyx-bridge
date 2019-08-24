set VERSION=%1
set OUT=.\dist\%VERSION%
echo Building %VERSION%...

rmdir /s /q %OUT%
mkdir %OUT%

:Windows
cargo build --manifest-path bridge/Cargo.toml --no-default-features --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --release
mkdir %OUT%\yyx-snapshot-%VERSION%-Windows
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-Windows\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-Windows\yyx-snapshot.exe

:MuMu
cargo build --manifest-path bridge/Cargo.toml --no-default-features --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="mumu noadmin" --release
mkdir %OUT%\yyx-snapshot-%VERSION%-Windows
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-MuMu\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-MuMu\yyx-snapshot.exe

:Steam
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=steam --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=noadmin --release
mkdir %OUT%\yyx-snapshot-%VERSION%-Steam
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-Steam\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-Steam\yyx-snapshot-Steam.exe

:FacebookGameroom
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=fg --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=noadmin --release
mkdir %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom\yyx-snapshot-FacebookGameroom.exe

:DMM
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=dmm --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --release
mkdir %OUT%\yyx-snapshot-%VERSION%-DMM
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-DMM\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-DMM\yyx-snapshot-DMM.exe

:GuildWindows
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=guild --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=guild --release
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-Windows
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-Windows\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-Windows\yyx-guildsnapshot.exe

:GuildSteam
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild steam" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild noadmin" --release
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-Steam
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-Steam\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-Steam\yyx-guildsnapshot-Steam.exe

:GuildFacebookGameroom
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild fg" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild noadmin" --release
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom\yyx-guildsnapshot-FacebookGameroom.exe

:GuildDMM
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild dmm" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild" --release
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-DMM
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-DMM\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-DMM\yyx-guildsnapshot-DMM.exe