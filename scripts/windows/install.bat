@echo off
setlocal

set BINARY_NAME=treki-cli.exe
set SOURCE=target\x86_64-pc-windows-gnu\release\%BINARY_NAME%
set INSTALL_DIR=%USERPROFILE%\AppData\Local\Programs\treki-cli

echo 🚀 Installing %BINARY_NAME%...

:: Check if the binary exists
if not exist "%SOURCE%" (
    echo ❌ Error: %SOURCE% not found.
    echo Please build the project first with: cargo build --release
    exit /b 1
)

:: Create the install directory if it doesn't exist
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

:: Copy the binary
copy "%SOURCE%" "%INSTALL_DIR%\%BINARY_NAME%" >nul

:: Check if install directory is in PATH
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if errorlevel 1 (
    echo 🔧 Adding %INSTALL_DIR% to PATH...
    setx PATH "%PATH%;%INSTALL_DIR%" >nul
)

echo ✅ Installed %BINARY_NAME% to %INSTALL_DIR%
echo 🎉 You can now use it from any terminal (restart terminal if needed).

endlocal
pause

