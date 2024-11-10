@echo off
title Solana Copy Trader
color 0A

:: Enable ANSI escape codes for colored output
reg add HKEY_CURRENT_USER\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>nul

echo [92m================================[0m
echo [96m    Solana Copy Trader v1.0[0m
echo [92m================================[0m
echo.

:: Set environment variables
set RUST_LOG=info

:: Check if cargo is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [91mError: Rust/Cargo is not installed![0m
    echo Please install Rust from https://rustup.rs/
    echo.
    pause
    exit /b 1
)

:: Check if .env exists
if not exist .env (
    echo [91mError: .env file not found![0m
    echo Please create .env file with required configuration.
    echo.
    pause
    exit /b 1
)

:: Build project
echo [93mBuilding project...[0m
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo [91mError: Build failed![0m
    echo Please check the error messages above.
    echo.
    pause
    exit /b 1
)

:: Clear screen and show running message
cls
echo [92m================================[0m
echo [96m    Solana Copy Trader v1.0[0m
echo [92m================================[0m
echo.
echo [92mTrader is now running...[0m
echo [93mPress Ctrl+C to stop[0m
echo.

:: Run the trader
cargo run --release

echo.
echo [91mTrader has stopped.[0m
pause