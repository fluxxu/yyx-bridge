set VERSION=%1
set OUT=.\dist\%VERSION%
echo Building %VERSION%...

rmdir /s /q %OUT%
mkdir %OUT%

:MuMu
cargo build --manifest-path bridge/Cargo.toml --no-default-features --release
cargo build --manifest-path bridge-loader-windows/Cargo.toml --no-default-features --features="mumu noadmin" --release
mkdir %OUT%\yyx-snapshot-%VERSION%-MuMu
copy .\target\release\bridge.dll %OUT%\yyx-snapshot-%VERSION%-MuMu\bridge.dll
copy .\target\release\bridge-loader-windows.exe %OUT%\yyx-snapshot-%VERSION%-MuMu\yyx-snapshot.exe