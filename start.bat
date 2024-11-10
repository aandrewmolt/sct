@echo off
title Solana Copy Trader

:: Kill any existing instances
taskkill /F /IM solana-copy-trader.exe 2>nul

:: Check if cargo is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Rust/Cargo is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

:: Clean and rebuild if needed
echo Cleaning old builds...
cargo clean

:: Build the project
echo Building project...
cargo build --release

:: Run the trader with the run_trader.bat
echo Starting trader...
call run_trader.bat

pause 