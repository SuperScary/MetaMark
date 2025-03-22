@echo off
setlocal

:: Create build directory
if not exist build mkdir build
cd build

:: Configure with CMake
cmake .. -G "Visual Studio 17 2022" -A x64

:: Build
cmake --build . --config Release

:: Create Release directory if it doesn't exist
if not exist ..\Release mkdir ..\Release

:: Copy files to expected locations
copy /Y Release\metamark-core.lib ..\Release\
copy /Y Release\metamark-core.pdb ..\Release\

cd .. 