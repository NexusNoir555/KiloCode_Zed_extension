# KiloCode for Zed

AI-powered coding assistant for Zed editor. Ask questions, explain code, generate code, refactor, fix bugs, and generate documentation. Works with OpenAI-compatible APIs including free providers like Groq, Together AI, and OpenRouter.

## Features

- **6 Slash Commands** for different AI tasks
- **OpenAI-Compatible API** - Works with multiple AI providers
- **Free Provider Support** - Use free tiers from Groq, Together AI, OpenRouter, and more
- **Secure** - HTTPS-only communication, no code execution
- **Configurable** - Use your own API endpoint and credentials
- **Open Source** - Fully transparent and auditable

## Installation

### From Zed Extensions Registry (Recommended)

> **⚠️ Note**: Installation from the Zed Extensions Registry is currently **not available**. This feature is on our to-do list and will be implemented in a future release. For now, please use the manual installation method below.

### Manual Installation (Step-by-Step)

Follow these steps to install KiloCode manually:

#### Step 1: Download the Extension

1. Go to the [GitHub Releases page](https://github.com/NexusNoir555/KiloCode_Zed_extension/releases)
2. Find the latest release (it will be at the top of the page)
3. Click on the file named `kilocode-zed-extension-v0.1.0.zip` (or similar)
4. The file will download to your computer

#### Step 2: Find Your Zed Extensions Directory

The location depends on your operating system:

**For macOS:**
1. Open Finder
2. Press `Cmd + Shift + G` to open "Go to Folder"
3. Type: `~/.config/zed/extensions/`
4. Press Enter
5. If the `extensions` folder doesn't exist, create it by right-clicking and choosing "New Folder"

**For Linux:**
1. Open your file manager
2. Press `Ctrl + L` to type a location
3. Type: `~/.config/zed/extensions/`
4. Press Enter
5. If the `extensions` folder doesn't exist, create it by right-clicking and choosing "New Folder"

**For Windows:**
1. Press `Win + R` to open the Run dialog
2. Type: `%APPDATA%\Zed\extensions\`
3. Press Enter
4. If the `extensions` folder doesn't exist, create it by right-clicking and choosing "New Folder"

#### Step 3: Extract and Move the Extension

1. Find the downloaded file (usually in your Downloads folder)
2. Right-click on the file and choose "Extract All" or "Unzip"
3. Open the extracted folder
4. You should see a folder named `kilocode-zed-extension`
5. Copy this entire folder
6. Paste it into your Zed extensions directory (from Step 2)

#### Step 4: Restart Zed

1. Close Zed completely
2. Open Zed again
3. The extension should now be loaded!

#### Step 5: Verify Installation

1. Open Zed
2. Press `Cmd + Shift + P` (macOS) or `Ctrl + Shift + P` (Linux/Windows)
3. Type "Extensions: List"
4. Press Enter
5. You should see "KiloCode" in the list of extensions

### Building from Source (For Developers)

If you want to build the extension yourself:

1. Make sure you have Rust installed
2. Clone the repository:
   ```bash
   git clone https://github.com/NexusNoir555/KiloCode_Zed_extension.git
   cd KiloCode_Zed_extension
   ```
3. Build the extension:
   ```bash
   cargo build --release
   ```
4. The compiled extension will be in the `target/release/` directory
5. Copy the extension folder to your Zed extensions directory (see Step 2 above)

## Configuration

Before using KiloCode, you need to configure your API credentials. KiloCode supports **OpenAI-compatible APIs**, which means you can use it with many different AI providers, including free options!

### Supported AI Providers

KiloCode works with any OpenAI-compatible API. Here are some popular options:

| Provider | Free Tier | API URL | Recommended Models |
|----------|-----------|---------|-------------------|
| **Groq** | ✅ Yes (generous) | `https://api.groq.com/openai/v1` | `llama3-70b-8192`, `mixtral-8x7b-32768` |
| **Together AI** | ✅ Yes (limited) | `https://api.together.xyz/v1` | `meta-llama/Llama-3-70b-chat-hf` |
| **OpenRouter** | ✅ Yes (some models) | `https://openrouter.ai/api/v1` | Various free models available |
| **OpenAI** | ❌ No | `https://api.openai.com/v1` | `gpt-4`, `gpt-3.5-turbo` |
| **Anthropic** | ❌ No | `https://api.anthropic.com/v1` | `claude-3-opus`, `claude-3-sonnet` |
| **DeepSeek** | ✅ Yes | `https://api.deepseek.com/v1` | `deepseek-chat` |
| **Hugging Face** | ✅ Yes (limited) | `https://api-inference.huggingface.co/v1` | Various models |

> **💡 Recommendation**: For free usage, we recommend **Groq** - it offers a generous free tier with very fast inference using open-source models like Llama 3.

### Step 1: Get Your API Key

Choose a provider from the table above and get your API key:

#### For Groq (Recommended - Free):

1. Go to [https://console.groq.com/](https://console.groq.com/)
2. Sign up for a free account
3. Go to "API Keys" in the left sidebar
4. Click "Create API Key"
5. Copy the API key

#### For Together AI (Free Tier):

1. Go to [https://api.together.xyz/](https://api.together.xyz/)
2. Sign up for a free account
3. Go to "API Keys" in your dashboard
4. Copy your API key

#### For OpenRouter (Some Free Models):

1. Go to [https://openrouter.ai/](https://openrouter.ai/)
2. Sign up for a free account
3. Go to "Keys" in your dashboard
4. Copy your API key

#### For OpenAI (Paid):

1. Go to [https://platform.openai.com/](https://platform.openai.com/)
2. Sign up or log in
3. Go to "API Keys" in your dashboard
4. Click "Create new secret key"
5. Copy the API key

**Important**: Keep your API key secret! Don't share it with anyone.

### Step 2: Set the API Key (macOS)

**Option A: Temporary (only works for this session)**

1. Open your Terminal app (you can find it in Applications → Utilities)
2. Type this command and press Enter:
   ```bash
   export KILOCODE_API_KEY="paste-your-api-key-here"
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Now start Zed by typing `zed` and pressing Enter

**Option B: Permanent (works every time you open Zed)**

1. Open your Terminal app
2. Type this command and press Enter:
   ```bash
   echo 'export KILOCODE_API_KEY="paste-your-api-key-here"' >> ~/.zshrc
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Close your Terminal
5. Open Terminal again and type `source ~/.zshrc` to apply the changes
6. Now you can start Zed anytime and it will have your API key!

### Step 2: Set the API Key (Linux)

**Option A: Temporary (only works for this session)**

1. Open your Terminal
2. Type this command and press Enter:
   ```bash
   export KILOCODE_API_KEY="paste-your-api-key-here"
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Now start Zed by typing `zed` and pressing Enter

**Option B: Permanent (works every time you open Zed)**

1. Open your Terminal
2. Type this command and press Enter:
   ```bash
   echo 'export KILOCODE_API_KEY="paste-your-api-key-here"' >> ~/.bashrc
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Close your Terminal
5. Open Terminal again and type `source ~/.bashrc` to apply the changes
6. Now you can start Zed anytime and it will have your API key!

### Step 2: Set the API Key (Windows)

**Option A: Temporary (only works for this session) - Using PowerShell**

1. Press `Win + X` and choose "Windows PowerShell"
2. Type this command and press Enter:
   ```powershell
   $env:KILOCODE_API_KEY="paste-your-api-key-here"
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Now start Zed by typing `zed` and pressing Enter

**Option B: Temporary (only works for this session) - Using Command Prompt**

1. Press `Win + R`, type `cmd`, and press Enter
2. Type this command and press Enter:
   ```cmd
   set KILOCODE_API_KEY=paste-your-api-key-here
   ```
3. Replace `paste-your-api-key-here` with your actual API key
4. Now start Zed by typing `zed` and pressing Enter

**Option C: Permanent (works every time you open Zed)**

1. Press `Win + R`, type `sysdm.cpl`, and press Enter
2. Click the "Advanced" tab
3. Click "Environment Variables"
4. Under "User variables", click "New"
5. For "Variable name", type: `KILOCODE_API_KEY`
6. For "Variable value", paste your API key
7. Click "OK" on all windows
8. Close and reopen Zed

### Step 3: Verify Your API Key is Set

**macOS/Linux:**
1. Open your Terminal
2. Type: `echo $KILOCODE_API_KEY`
3. Press Enter
4. You should see your API key printed on the screen

**Windows (PowerShell):**
1. Open PowerShell
2. Type: `echo $env:KILOCODE_API_KEY`
3. Press Enter
4. You should see your API key printed on the screen

**Windows (Command Prompt):**
1. Open Command Prompt
2. Type: `echo %KILOCODE_API_KEY%`
3. Press Enter
4. You should see your API key printed on the screen

### Configuration Options

| Setting | Description | Default | Required |
|---------|-------------|---------|----------|
| `KILOCODE_API_KEY` | Your API key from chosen provider | - | **Yes** |
| `KILOCODE_API_URL` | API endpoint URL | `https://api.openai.com/v1` | No |
| `KILOCODE_MODEL` | Model name to use | `gpt-3.5-turbo` | No |

### Example Configurations

#### Groq (Free - Recommended):

**macOS/Linux:**
```bash
export KILOCODE_API_KEY="your-groq-api-key"
export KILOCODE_API_URL="https://api.groq.com/openai/v1"
export KILOCODE_MODEL="llama3-70b-8192"
```

**Windows (PowerShell):**
```powershell
$env:KILOCODE_API_KEY="your-groq-api-key"
$env:KILOCODE_API_URL="https://api.groq.com/openai/v1"
$env:KILOCODE_MODEL="llama3-70b-8192"
```

#### Together AI (Free Tier):

**macOS/Linux:**
```bash
export KILOCODE_API_KEY="your-together-api-key"
export KILOCODE_API_URL="https://api.together.xyz/v1"
export KILOCODE_MODEL="meta-llama/Llama-3-70b-chat-hf"
```

**Windows (PowerShell):**
```powershell
$env:KILOCODE_API_KEY="your-together-api-key"
$env:KILOCODE_API_URL="https://api.together.xyz/v1"
$env:KILOCODE_MODEL="meta-llama/Llama-3-70b-chat-hf"
```

#### OpenRouter (Some Free Models):

**macOS/Linux:**
```bash
export KILOCODE_API_KEY="your-openrouter-api-key"
export KILOCODE_API_URL="https://openrouter.ai/api/v1"
export KILOCODE_MODEL="meta-llama/llama-3-8b-instruct:free"
```

**Windows (PowerShell):**
```powershell
$env:KILOCODE_API_KEY="your-openrouter-api-key"
$env:KILOCODE_API_URL="https://openrouter.ai/api/v1"
$env:KILOCODE_MODEL="meta-llama/llama-3-8b-instruct:free"
```

#### OpenAI (Paid):

**macOS/Linux:**
```bash
export KILOCODE_API_KEY="your-openai-api-key"
export KILOCODE_API_URL="https://api.openai.com/v1"
export KILOCODE_MODEL="gpt-4"
```

**Windows (PowerShell):**
```powershell
$env:KILOCODE_API_KEY="your-openai-api-key"
$env:KILOCODE_API_URL="https://api.openai.com/v1"
$env:KILOCODE_MODEL="gpt-4"
```

## Usage

### How to Use KiloCode (Step-by-Step)

KiloCode works through Zed's built-in Assistant. Here's how to use it:

#### Step 1: Open Zed's Assistant

1. Open Zed
2. Press `Cmd + Shift + A` (macOS) or `Ctrl + Shift + A` (Linux/Windows)
3. The Assistant panel will open on the right side of your screen

#### Step 2: Type a Slash Command

In the Assistant panel, type one of the commands below and press Enter:

| Command | What it does | Example |
|---------|-------------|---------|
| `/kc` | Ask any coding question | `/kc How do I write a for loop?` |
| `/kc-explain` | Explain code you've selected | `/kc-explain` |
| `/kc-generate` | Create new code from a description | `/kc-generate Create a login function` |
| `/kc-refactor` | Improve code you've selected | `/kc-refactor` |
| `/kc-fix` | Find and fix bugs in your code | `/kc-fix` |
| `/kc-docs` | Add documentation to your code | `/kc-docs` |

#### Step 3: Wait for the Response

After you press Enter, KiloCode will:
1. Send your request to the API
2. Wait for the AI to process it
3. Display the response in the Assistant panel

### Detailed Examples

#### Example 1: Ask a Coding Question

1. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
2. Type: `/kc What's the difference between let and const in JavaScript?`
3. Press Enter
4. Wait for the answer to appear

#### Example 2: Explain Code

1. Open a file in Zed
2. Use your mouse to select the code you want to understand
3. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
4. Type: `/kc-explain`
5. Press Enter
6. KiloCode will explain what the selected code does

#### Example 3: Generate New Code

1. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
2. Type: `/kc-generate Create a Python function that sorts a list of numbers`
3. Press Enter
4. KiloCode will generate the Python code for you
5. You can copy the code and paste it into your file

#### Example 4: Refactor Code

1. Open a file in Zed
2. Select the code you want to improve
3. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
4. Type: `/kc-refactor Make this code more readable`
5. Press Enter
6. KiloCode will suggest improvements

#### Example 5: Fix Bugs

1. Open a file in Zed
2. Select the code that has bugs
3. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
4. Type: `/kc-fix`
5. Press Enter
6. KiloCode will find and fix the bugs

#### Example 6: Generate Documentation

1. Open a file in Zed
2. Select the code you want to document
3. Open Zed's Assistant (`Cmd + Shift + A` or `Ctrl + Shift + A`)
4. Type: `/kc-docs`
5. Press Enter
6. KiloCode will generate documentation for your code

### Tips for Better Results

- **Be specific**: Instead of "help me", try "how do I create a REST API in Rust?"
- **Provide context**: When asking about code, select the relevant code first
- **Use examples**: "Create a function like this one but for Python" works better than just "create a function"
- **Ask follow-up questions**: You can keep asking questions in the same conversation

## Security

KiloCode is designed with security in mind:

- ✅ **HTTPS-only** communication with API endpoints
- ✅ **No code execution** - AI responses are text only
- ✅ **Input validation** - Prevents injection attacks
- ✅ **Output sanitization** - Removes potentially harmful content
- ✅ **Open source** - Fully auditable code
- ✅ **User-controlled credentials** - You provide your own API key

See [SECURITY.md](SECURITY.md) for more details.

## Troubleshooting

### Problem: "KILOCODE_API_KEY environment variable not set" error

**What this means:** Zed can't find your API key.

**How to fix it:**

**Step 1:** Check if your API key is set
- **macOS/Linux:** Open Terminal and type: `echo $KILOCODE_API_KEY`
- **Windows (PowerShell):** Type: `echo $env:KILOCODE_API_KEY`
- **Windows (Command Prompt):** Type: `echo %KILOCODE_API_KEY%`

**Step 2:** If nothing appears, set your API key again
- Follow the "Configuration" section above to set your API key

**Step 3:** Close Zed completely and reopen it
- Make sure to close ALL Zed windows
- Open Zed again
- Try using KiloCode again

### Problem: "API request failed" error

**What this means:** The extension couldn't connect to the API.

**How to fix it:**

**Step 1:** Check your internet connection
- Try opening a web browser
- If websites don't load, you have no internet connection

**Step 2:** Check if your API key is correct
- Go to your provider's dashboard (Groq, Together AI, OpenRouter, etc.)
- Check your API key in the settings
- Make sure you copied it correctly (no extra spaces)

**Step 3:** Check your API URL and model
- Make sure `KILOCODE_API_URL` is set correctly for your provider
- Make sure `KILOCODE_MODEL` is a valid model for your provider
- See the "Example Configurations" section above for correct values

**Step 4:** Try a different API key
- Generate a new API key from your provider's dashboard
- Update your environment variable with the new key
- Restart Zed and try again

### Problem: Extension not showing up

**What this means:** Zed can't find the KiloCode extension.

**How to fix it:**

**Step 1:** Check if the extension is in the right folder
- **macOS:** Open Finder and go to `~/.config/zed/extensions/`
- **Linux:** Open your file manager and go to `~/.config/zed/extensions/`
- **Windows:** Open File Explorer and go to `%APPDATA%\Zed\extensions\`
- You should see a folder named `kilocode-zed-extension`

**Step 2:** If the folder is not there, reinstall the extension
- Follow the "Installation" section above

**Step 3:** Restart Zed
- Close ALL Zed windows
- Open Zed again

**Step 4:** Check the extension list
- Press `Cmd + Shift + P` (macOS) or `Ctrl + Shift + P` (Linux/Windows)
- Type "Extensions: List"
- Press Enter
- Look for "KiloCode" in the list

### Problem: Commands not working

**What this means:** The extension is installed but commands don't work.

**How to fix it:**

**Step 1:** Make sure your API key is set
- Follow the steps in the "KILOCODE_API_KEY environment variable not set" section above

**Step 2:** Check Zed's console for error messages
- Press `Cmd + Shift + P` (macOS) or `Ctrl + Shift + P` (Linux/Windows)
- Type "Toggle Developer Console"
- Press Enter
- Look for any red error messages
- Copy the error message and search for it online or report it

**Step 3:** Make sure you're using the Assistant correctly
- Press `Cmd + Shift + A` (macOS) or `Ctrl + Shift + A` (Linux/Windows)
- Type your command starting with `/`
- Press Enter

### Problem: Response is very slow

**What this means:** The API is taking a long time to respond.

**How to fix it:**

**Step 1:** Check your internet connection
- A slow connection can cause slow responses

**Step 2:** Try a simpler question
- Complex questions take longer to process
- Try something simple like "What is 2+2?"

**Step 3:** Wait a bit longer
- Sometimes the API is busy
- Give it up to 30 seconds

### Still having problems?

If none of these solutions work, please:

1. Check the [GitHub Issues page](https://github.com/NexusNoir555/KiloCode_Zed_extension/issues)
2. See if someone else has the same problem
3. If not, create a new issue and include:
   - Your operating system (macOS, Linux, or Windows)
   - The error message you're seeing
   - What you were trying to do when the error happened

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- 📖 [Documentation](https://github.com/NexusNoir555/KiloCode_Zed_extension/wiki)
- 🐛 [Report Issues](https://github.com/NexusNoir555/KiloCode_Zed_extension/issues)
- 💬 [Discussions](https://github.com/NexusNoir555/KiloCode_Zed_extension/discussions)

## Acknowledgments

- Built with [Zed Extension API](https://github.com/zed-industries/zed)
- Uses OpenAI-compatible API format for broad provider support
- Free providers supported: [Groq](https://groq.com/), [Together AI](https://together.ai/), [OpenRouter](https://openrouter.ai/)
- Inspired by the KiloCode VS Code extension
