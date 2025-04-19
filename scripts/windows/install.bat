@echo off
setlocal

set DOWNLOAD_URL=https://github.com/aether-flux/treki-cli/releases/download/v1.0.0/treki-cli.exe
set BINARY_NAME=treki-cli.exe
set FINAL_NAME=treki.exe
set INSTALL_DIR=%USERPROFILE%\AppData\Local\Programs\treki-cli
set DEST=%INSTALL_DIR%\%FINAL_NAME%

echo 🚀 Installing Treki CLI...

:: Create the install directory if it doesn't exist
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

:: Download the binary
echo ⬇️ Downloading binary from GitHub...
powershell -Command "Invoke-WebRequest -Uri %DOWNLOAD_URL% -OutFile '%DEST%'"

:: Check if the binary was downloaded
if not exist "%DEST%" (
    echo ❌ Failed to download the binary.
    exit /b 1
)

:: Check if install directory is in PATH
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if errorlevel 1 (
    echo 🔧 Adding %INSTALL_DIR% to PATH...
    setx PATH "%PATH%;%INSTALL_DIR%" >nul
)

echo ✅ Installed as 'treki' in %INSTALL_DIR%
echo 🎉 You can now use it from any terminal (restart terminal if needed).

endlocal
pause

