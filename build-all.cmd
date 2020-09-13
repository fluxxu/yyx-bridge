set VERSION=%1
set OUT=.\dist\%VERSION%
echo Building %VERSION%...

rmdir /s /q %OUT%
mkdir %OUT%

:Windows
cargo build --manifest-path bridge/Cargo.toml --no-default-features --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-snapshot-%VERSION%-Windows
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-Windows\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-Windows\yyx-snapshot.exe
7z a %OUT%\yyx-snapshot-%VERSION%-Windows.zip %OUT%\yyx-snapshot-%VERSION%-Windows

:MuMu
cargo build --manifest-path bridge/Cargo.toml --no-default-features --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="mumu noadmin" --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-snapshot-%VERSION%-MuMu
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-MuMu\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-MuMu\yyx-snapshot.exe
7z a %OUT%\yyx-snapshot-%VERSION%-MuMu.zip %OUT%\yyx-snapshot-%VERSION%-MuMu

:Steam
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=steam --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=noadmin --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-snapshot-%VERSION%-Steam
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-Steam\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-Steam\yyx-snapshot-Steam.exe
7z a %OUT%\yyx-snapshot-%VERSION%-Steam.zip %OUT%\yyx-snapshot-%VERSION%-Steam

:FacebookGameroom
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=fg --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=noadmin --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom\yyx-snapshot-FacebookGameroom.exe
7z a %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom.zip %OUT%\yyx-snapshot-%VERSION%-FacebookGameroom

:DMM
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=dmm --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-snapshot-%VERSION%-DMM
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-DMM\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-DMM\yyx-snapshot-DMM.exe
7z a %OUT%\yyx-snapshot-%VERSION%-DMM.zip %OUT%\yyx-snapshot-%VERSION%-DMM

:GuildWindows
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=guild --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features=guild --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-Windows
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-Windows\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-Windows\yyx-guildsnapshot.exe
7z a %OUT%\yyx-guildsnapshot-%VERSION%-Windows.zip %OUT%\yyx-guildsnapshot-%VERSION%-Windows

:GuildMuMu
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features=guild --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="mumu noadmin" --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-MuMu
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-MuMu\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-MuMu\yyx-guildsnapshot.exe
7z a %OUT%\yyx-guildsnapshot-%VERSION%-MuMu.zip %OUT%\yyx-guildsnapshot-%VERSION%-MuMu

:GuildSteam
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild steam" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild noadmin" --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-Steam
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-Steam\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-Steam\yyx-guildsnapshot-Steam.exe
7z a %OUT%\yyx-guildsnapshot-%VERSION%-Steam.zip %OUT%\yyx-guildsnapshot-%VERSION%-Steam

:GuildFacebookGameroom
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild fg" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild noadmin" --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom\yyx-guildsnapshot-FacebookGameroom.exe
7z a %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom.zip %OUT%\yyx-guildsnapshot-%VERSION%-FacebookGameroom

:GuildDMM
cargo build --manifest-path bridge/Cargo.toml --no-default-features --features="guild dmm" --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="guild" --release
signtool sign /a .\target\release\bridge.dll
signtool sign /a .\target\release\bridge-loader-windows.exe
mkdir %OUT%\yyx-guildsnapshot-%VERSION%-DMM
copy .\target\release\bridge.dll %OUT%\yyx-guildsnapshot-%VERSION%-DMM\bridge-guild.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-guildsnapshot-%VERSION%-DMM\yyx-guildsnapshot-DMM.exe
7z a %OUT%\yyx-guildsnapshot-%VERSION%-DMM.zip %OUT%\yyx-guildsnapshot-%VERSION%-DMM
