@echo off

:: Run cargo check
cargo check
set CHECK_EXIT=%ERRORLEVEL%

:: Only run cargo run if cargo check succeeds
if %CHECK_EXIT% EQU 0 (
    cargo run
) else (
    echo Cargo check failed. Not running.
)
