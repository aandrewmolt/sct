@echo off
echo Setting up Git repository...

:: Remove existing git setup if any
echo Cleaning existing git setup...
rmdir /s /q .git
git remote remove origin

:: Initialize git and create README
echo "# sct" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main

:: Set the correct remote URL
git remote add origin https://github.com/aandrewmolt/sct.git

:: Add all other files
git add .
git add -f .env.example

:: Commit all files
git commit -m "Initial commit: Solana copy trading bot"

:: Push to GitHub
echo.
echo Before pushing, please make sure you have:
echo 1. Created the repository at: https://github.com/aandrewmolt/sct
echo 2. Generated a GitHub token with 'repo' scope
echo.
echo Press any key when ready...
pause >nul

git push -u origin main --force

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Push failed. Please check:
    echo 1. You have write access to the repository
    echo 2. You might need to generate a GitHub token:
    echo    - Go to GitHub Settings
    echo    - Developer Settings
    echo    - Personal Access Tokens (classic)
    echo    - Generate new token
    echo    - Select 'repo' scope
    echo    - Use token as password when prompted
)

echo.
echo Done! Check your GitHub repository.
pause