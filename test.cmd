@echo off

set "binname=lumina"
set "binpath=.\target\x86_64-linux-android\release\%binname%"
set "emul=Pixel_7_Pro_API_34"
set "dest=/data/local/tmp/"

if not exist %binpath% (
    echo Binary does not exists: %binpath%
    exit 1
)

echo Binary found: %binpath%

adb devices | findstr "emulator" > nul

if errorlevel 1 (
    echo Emulator not running!
    echo Starting emulator using flutter...
    flutter emulator --launch %emul%
    if errorlevel 1 (
        echo Failed to start flutter emulator, terminating...
        exit 1
    )
) else (
    echo An emulator is running!
)

adb push %binpath% %dest%

adb shell "cd %dest%; ls" | findstr "%binname%" > nul

if errorlevel 1 (
    echo "Binary not found on emulator: %dest%%binname%"
    exit 1
)

adb shell "chmod +x %dest%%binname%; %dest%%binname%"