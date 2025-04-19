@echo off
setlocal

set DOWNLOAD_URL=https://github.com/aether-flux/treki-cli/releases/download/v1.0.0/treki-cli.exe
set BINARY_NAME=treki-cli.exe
set FINAL_NAME=treki.exe
set INSTALL_DIR=%USERPROFILE%\AppData\Local\Programs\treki-cli
set DEST=%INSTALL_DIR%\%FINAL_NAME%

echo 🚀 Installing Treki...

:: Create install directory if it doesn't exist
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

echo ⬇️ Downloading binary from GitHub...

:: Prefer curl if available
where curl >nul 2>&1
if %errorlevel%==0 (
    curl -L "%DOWNLOAD_URL%" -o "%DEST%"
) else (
    powershell -Command "Start-BitsTransfer -Source '%DOWNLOAD_URL%' -Destination '%DEST%'"
)

:: Check if the binary was downloaded
if not exist "%DEST%" (
    echo ❌ Failed to download the binary.
    exit /b 1
)

:: Add to PATH if not already in it
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if errorlevel 1 (
    echo 🔧 Adding to PATH...
    setx PATH "%PATH%;%INSTALL_DIR%" >nul
)

echo ✅ Installed as 'treki' in %INSTALL_DIR%
echo 🎉 You can now use it from any terminal (restart terminal if needed).

endlocal
pause

