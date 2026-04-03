@echo off
cd /d C:\Users\Bobbi\code\orqastudio-dev\app
npx vitest run --reporter=verbose 2>&1 > test-results.txt
echo EXIT_CODE: %ERRORLEVEL% >> test-results.txt
