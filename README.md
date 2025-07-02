# Workshop: Supercharge Your Development - AI in the Coding Workflow

## Git Installation

### Installing Git on Mac

#### Method 1: Using Homebrew (Recommended)

First, install Homebrew if you don't have it:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install Git using Homebrew:

```bash
brew install git
```

#### Method 2: Using the Official Installer

1. Go to https://git-scm.com/download/mac
2. Download the installer for your Mac (Intel or Apple Silicon)
3. Open the downloaded .dmg file
4. Run the installer and follow the prompts

#### Method 3: Using Xcode Command Line Tools

1. Open Terminal
2. Run: `git --version`
3. If Git isn't installed, macOS will prompt you to install Xcode Command Line Tools
4. Click "Install" and follow the prompts

### Installing Git on Windows

#### Method 1: Using the Official Installer (Recommended)

1. Go to https://git-scm.com/download/windows
2. Download the installer (64-bit or 32-bit based on your system)
3. Run the installer
4. Follow the setup wizard with these recommended settings:
   - Choose default editor (VS Code if you have it, otherwise Notepad++)
   - Select "Git from the command line and also from 3rd-party software"
   - Choose "Use bundled OpenSSH"
   - Select "Use the OpenSSL library"
   - Choose "Checkout Windows-style, commit Unix-style line endings"
   - Select "Use Windows' default console window"
   - Enable "Enable file system caching" and "Enable Git Credential Manager"

#### Method 2: Using Chocolatey

Install Chocolatey if you don't have it (run as Administrator):

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

Install Git:

```powershell
choco install git
```

#### Method 3: Using Winget (Windows 10+)

1. Open Command Prompt or PowerShell
2. Run: `winget install --id Git.Git -e --source winget`

### Verification

After installation on either platform, verify Git is installed correctly:

1. Open Terminal (Mac) or Command Prompt/PowerShell (Windows)
2. Run: `git --version`
3. You should see the Git version number

### Initial Configuration

After installation, configure Git with your information:

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

> **Note:** The official installer method is generally recommended for both platforms as it provides the most recent stable version and includes Git Bash on Windows, which gives you a Unix-like command line experience.

---

## Rust Installation

### Mac Installation

#### Method 1: Using rustup (Recommended)

1. Open Terminal
2. Run this command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the on-screen prompts (usually just press Enter to accept defaults)
4. Restart your terminal or run:

```bash
source ~/.cargo/env
```

#### Method 2: Using Homebrew

```bash
brew install rust
```

### Windows Installation

#### Method 1: Using rustup (Recommended)

1. Go to https://rustup.rs/
2. Download and run rustup-init.exe
3. Follow the installer prompts
4. Restart your command prompt or PowerShell

#### Method 2: Using winget

```powershell
winget install Rustlang.Rust.MSVC
```

#### Method 3: Using Chocolatey

```powershell
choco install rust
```

### Verify Installation

After installation on either platform, verify Rust is installed correctly:

```bash
rustc --version
cargo --version
```

You should see version information for both the Rust compiler (rustc) and package manager (cargo).

### Next Steps

Create your first project:

```bash
cargo new hello_world
cd hello_world
cargo run
```

Update Rust (when needed):

```bash
rustup update
```

Install additional components:

```bash
rustup component add clippy    # Linter
rustup component add rustfmt   # Code formatter
```

> **Note:** The rustup method is recommended because it includes the Rust toolchain manager, making it easy to update Rust and manage different versions if needed.

---

## Node.js Installation Guide

This guide will walk you through installing Node.js from the official website at https://nodejs.org/en/download

### Prerequisites

Before installing Node.js, ensure you have administrative privileges on your computer and a stable internet connection.

### Step 1: Visit the Official Node.js Website

1. Open your web browser and navigate to: https://nodejs.org/en/download
2. You'll see the Node.js download page with various installation options

### Step 2: Choose Your Installation Method

The Node.js website offers several installation options:

#### Option A: LTS (Long Term Support) - Recommended for Most Users

- Look for the LTS (Long Term Support) version - this is the most stable version
- The website will automatically detect your operating system
- Click the large download button for your operating system

#### Option B: Current Version

- If you want the latest features, select the Current version
- Note that this version may have newer features but could be less stable

### Step 3: Operating System Specific Instructions

#### Windows Installation

1. **Download the Windows Installer (.msi)**
   - Choose between 32-bit or 64-bit (most modern computers use 64-bit)
   - If unsure, check your system: Right-click "This PC" → Properties → System type

2. **Run the Installer**
   - Double-click the downloaded .msi file
   - Follow the installation wizard:
     - Accept the license agreement
     - Choose installation directory (default is recommended)
     - Select components (keep default selections)
     - Complete the installation

3. **Verify Installation**
   - Open Command Prompt (cmd) or PowerShell
   - Type: `node --version`
   - Type: `npm --version`
   - Both commands should return version numbers

#### macOS Installation

1. **Download the macOS Installer (.pkg)**
   - The website will detect if you're using an Intel or Apple Silicon Mac
   - Download the appropriate version

2. **Run the Installer**
   - Double-click the downloaded .pkg file
   - Follow the installation wizard
   - Enter your administrator password when prompted

3. **Verify Installation**
   - Open Terminal
   - Type: `node --version`
   - Type: `npm --version`
   - Both commands should return version numbers

#### Linux Installation

##### Method 1: Download Pre-built Binaries

1. **Download Linux Binaries**
   - Choose between 32-bit or 64-bit
   - Download the .tar.xz file

2. **Extract and Install**

```bash
# Navigate to downloads directory
cd ~/Downloads

# Extract the archive (replace with your downloaded filename)
tar -xf node-v[version]-linux-x64.tar.xz

# Move to a system directory
sudo mv node-v[version]-linux-x64 /usr/local/nodejs

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH=/usr/local/nodejs/bin:$PATH

# Reload your shell configuration
source ~/.bashrc
```

##### Method 2: Using Package Manager

```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# CentOS/RHEL/Fedora
curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash -
sudo yum install -y nodejs
```

**Verify Installation:**

- Open Terminal
- Type: `node --version`
- Type: `npm --version`

### Alternative Installation Methods

#### Using Node Version Manager (NVM)

NVM allows you to install and manage multiple Node.js versions:

**For macOS/Linux:**

```bash
# Install NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Restart terminal or run:
source ~/.bashrc

# Install latest LTS Node.js
nvm install --lts

# Use the installed version
nvm use --lts
```

**For Windows:** Use nvm-windows from: https://github.com/coreybutler/nvm-windows

#### Using Package Managers

**Windows (Chocolatey):**

```cmd
choco install nodejs
```

**macOS (Homebrew):**

```bash
brew install node
```

**Linux (Snap):**

```bash
sudo snap install node --classic
```

### Verification and Testing

After installation, verify Node.js is working correctly:

**Check Versions:**

```bash
node --version
npm --version
```

**Create a Test File:**

```bash
# Create a simple test file
echo 'console.log("Hello, Node.js!");' > test.js

# Run the file
node test.js
```

**Test NPM:**

```bash
# Check NPM configuration
npm config list
```

### Troubleshooting

#### Common Issues:

1. **"Command not found" error:**
   - Restart your terminal/command prompt
   - Check if Node.js was added to your system PATH
   - Reinstall Node.js with administrator privileges

2. **Permission errors on macOS/Linux:**
   Configure npm to use a different directory for global packages:

```bash
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
# Add to ~/.bashrc: export PATH=~/.npm-global/bin:$PATH
```

3. **Old version still showing:**
   - Clear your terminal cache
   - Check if multiple Node.js versions are installed
   - Use NVM to manage versions

### Next Steps

After successful installation:

**Update NPM to latest version:**

```bash
npm install -g npm@latest
```

**Explore Node.js:**
- Try the Node.js REPL by typing `node` in terminal
- Install your first package: `npm install -g nodemon`
- Check out the official documentation: https://nodejs.org/en/docs/

**Set up your development environment:**
- Install a code editor (VS Code, Atom, etc.)
- Create your first Node.js project
- Learn about package.json and npm scripts

Congratulations! You now have Node.js installed and ready for development.

---

## Gemini CLI Installation

### Prerequisites

Ensure you have Node.js version 18 or higher installed.

### Installation and Setup

**Run the CLI:** Execute the following command in your terminal:

```bash
npx https://github.com/google-gemini/gemini-cli
```

Or install it with:

```bash
npm install -g @google/gemini-cli
gemini
```

1. Pick a color theme
2. **Authenticate:** When prompted, sign in with your personal Google account. This will grant you up to 60 model requests per minute and 1,000 model requests per day using Gemini.

You are now ready to use the Gemini CLI!

### Advanced Configuration

For advanced use or increased limits:

If you need to use a specific model or require a higher request capacity, you can use an API key:

1. Generate a key from Google AI Studio
2. Set it as an environment variable in your terminal. Replace YOUR_API_KEY with your generated key:

```bash
export GEMINI_API_KEY="YOUR_API_KEY"
```

For other authentication methods, including Google Workspace accounts, see the authentication guide.

### Install Context7 in Gemini CLI

See Gemini CLI Configuration for details.

1. Open the Gemini CLI settings file. The location is `~/.gemini/settings.json` (where ~ is your home directory).
2. Add the following to the mcpServers object in your settings.json file:

```json
{
  "mcpServers": {
    "context7": {
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp"]
    }
  }
}
```

---

## Todo App AI Development Guide

### Prompt Steps

**Run the CLI:**

```bash
npx https://github.com/google-gemini/gemini-cli
```

**Create a working directory** named `todoApp`

**First Prompt:**

### Developer Persona

Enter the below persona as the prompt:

#### Full-Stack Developer Persona: Kenneth Phang

**Profile Summary**  
Senior Full-Stack Developer | 6 years experience | Bukit Timah | Fintech startup

**Core Technical Stack**
- Backend: Rust (Actix-web, Axum, Tokio) - 3+ years
- Frontend: Angular + TypeScript (NgRx, RxJS) - 4+ years
- Database: PostgreSQL, MySQL, SQLite with advanced SQL skills
- Architecture: RESTful APIs, microservices, client-server patterns

**Key Expertise**
- Performance-focused: Memory safety, async programming, query optimization
- Type safety advocate: Leverages Rust and TypeScript for robust applications
- Full-stack architecture: Designs scalable systems from database to UI
- DevOps integration: Docker, CI/CD, cloud deployment (AWS/GCP)

**Professional Approach**
- Development: Test-driven, incremental delivery, code review focused
- Communication: Bridges technical and business stakeholders effectively
- Mentorship: Guides junior developers on best practices
- Learning: Active in open-source, conferences, and technical blogging

**Unique Value**  
Combines Rust's performance and safety with Angular's enterprise capabilities to build scalable, maintainable full-stack applications. Understands cross-stack implications and optimizes for both user experience and system performance.

### Project Requirements


**Second Prompt:**

#### Technical Stack
- **Frontend:** Angular application
- **Backend:** Rust server
- **Database:** SQLite for data persistence

#### Configuration Requirements
- Set up Angular proxy configuration file to redirect API calls to the backend server
- Configure CORS (Cross-Origin Resource Sharing) on the Rust backend server
- Ensure proper communication between frontend and backend running on different ports

#### Core CRUD Functionality
- **Create:** Add new todo items
- **Read:** Display/list all existing todos
- **Update:** Edit existing todo items
- **Delete:** Remove individual todo items

#### Todo Management Features
- Single todo deletion capability
- Todo item editing functionality
- Complete todo listing display
- New todo creation interface
- Todo completion status (checkable items to mark as completed)

#### Data Storage
- Store all todo information in SQLite database
- Maintain todo states (completed/incomplete)
- Persist data across application sessions

This setup will create a full-stack todo application with proper separation between frontend and backend, secure cross-origin communication, and complete task management capabilities. use context7

---

## Common Issues and Solutions

### Rust Backend Error

**Problem:** The error indicates that the `?` operator in your main function is trying to convert an `std::io::Error` (from `HttpServer::bind` and `HttpServer::run`) into a `rusqlite::Error`, but there's no direct conversion defined.

**Solution:** Change the return type of your main function to a more general error type that can encompass both `rusqlite::Error` and `std::io::Error`. A common way to do this is to use `Box<dyn std::error::Error>`.

### Angular Configuration

**Question:** Is it possible to configure the proxy config in the angular.json?

**Answer:** Yes, you can configure proxy settings in angular.json for the serve command.

### Angular Dependencies

**Task:** Install all the dependencies on the Angular frontend

### Angular Build Errors

**Error from Angular:**

```
Option "browserTarget" is deprecated: Use 'buildTarget' instead.
Application bundle generation failed. [1.235 seconds]
✘ [ERROR] TS2300: Duplicate identifier 'NgModule'. [plugin angular-compiler]
    src/app/app.module.ts:1:9:
      1 │ import { NgModule } from '@angular/core';
        ╵          ~~~~~~~~
✘ [ERROR] TS2300: Duplicate identifier 'BrowserModule'. [plugin angular-compiler]
    src/app/app.module.ts:2:9:
      2 │ import { BrowserModule } from '@angular/platform-browser';
        ╵          ~~~~~~~~~~~~~
```

### UI Enhancement Prompts

**third Prompt:**
```
- Integrate ng-zorro UI components and styling
- Center-align the todo canvas on the screen
- Improve the overall presentation and user experience
use context7
```
### Ng-Zorro Issues

**Problem:** Ng-zorro failed with multiple errors:

```
✘ [ERROR] TS1005: 'from' expected. [plugin angular-compiler]
    src/app/app.module.ts:2:20:
      2 │ import { NgModule } '@angular/core';
        ╵                     ~~~~~~~~~~~~~~~
✘ [ERROR] NG2: Type '"danger"' is not assignable to type 'NzButtonType'. [plugin angular-compiler]
✘ [ERROR] Could not resolve "~ng-zorro-antd/ng-zorro-antd.min.css"
```

### Missing Library Error

```
✘ [ERROR] Could not resolve "@ctrl/tinycolor"
    node_modules/ng-zorro-antd/fesm2022/ng-zorro-antd-core-config.mjs:5:26:
      5 │ import { TinyColor } from '@ctrl/tinycolor';
        ╵                           ~~~~~~~~~~~~~~~~~
```

### API Endpoint Alignment

**Task:** Align the URI to make sure the frontend is still calling `/api/todos`. Make changes to the Rust backend changing it from `/todos` to `/api/todos`.

### Method Not Allowed Error

**Task:** Fix the "method not allowed" error.

---

This workshop guide provides comprehensive instructions for setting up a full-stack development environment and building a todo application using modern tools and AI assistance.


### GIT PUSH YOUR CODE ! Kindly use the Gemini CLI not the git commands

### Project Context
Create a multi-stage Dockerfile for a full-stack todo application with the following specifications:

Application Architecture:
Frontend: Angular application (Node.js 20)
Backend: Rust web server using Actix-web framework
Database: SQLite (embedded, created at runtime)
Port: Application serves on port 8080

### Dockerfile Requirements

## Stage 1: Frontend Builder
Use node:20-alpine as base image
Build Angular application in production mode
Output static files to dist/frontend/browser/

## Stage 2: Backend Builder
Use rust:bookworm as base image
Install system dependencies required for rusqlite compilation (libssl-dev, pkg-config)
Implement dependency caching optimization (build deps before copying source)
Compile Rust application in release mode
Binary name should be backend

## Stage 3: Final Runtime Image
Use ubuntu:latest as minimal runtime base
Install only runtime dependencies (openssl, curl)
Set environment variable RUST_BACKTRACE=1 for debugging
Copy compiled backend binary to /app/bin/backend
Copy frontend static files to /app/static/
Configure backend to serve frontend static files
Expose port 8080
Add health check that tests /api/todos endpoint
Set appropriate working directory

## Optimization Requirements:
Multi-stage build to minimize final image size
Layer caching for dependencies (separate from source code)
Clean up package managers to reduce image size
Use appropriate base images for each stage
Runtime Behavior:
Backend serves both API endpoints (/api/*) and frontend static files
SQLite database created automatically at runtime (no need to copy)
Health monitoring via curl to API endpoint
Container should run the backend executable as the main process

## Generate a production-ready, optimized multi-stage Dockerfile that follows Docker best practices and efficiently builds both the Angular frontend and Rust backend into a single deployable container.