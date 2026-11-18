# Codex Advisor

Codex Advisor adds a second-opinion model to OpenAI Codex CLI 0.169.0. Choose the skill for on-demand consultations in an existing Codex installation. Install the Apple Silicon CLI for the native `/advisor` command and model-initiated consultations.

## Use the skill in your existing Codex

Paste the installation request into a Codex chat.

```text
$skill-installer Install the advisor skill from https://github.com/poirotw66/codex-advisor/tree/advisor-v0.1.0/skills/advisor
```

Open a new chat after installation. Use the skill to request a second opinion.

```text
$advisor Challenge the riskiest assumption in my plan before I implement it.
```

With stock Codex, the skill asks a separate subagent for advice when the client supports model selection for subagents. The subagent inherits the parent chat's permissions. [Skill instructions](./skills/advisor/SKILL.md)

## Install the CLI on an Apple Silicon Mac

Paste this block into a terminal to download, verify, and install the [v0.1.0 release](https://github.com/poirotw66/codex-advisor/releases/tag/advisor-v0.1.0).

```sh
(
  set -eu
  release_url=https://github.com/poirotw66/codex-advisor/releases/download/advisor-v0.1.0
  archive=codex-advisor-aarch64-apple-darwin.tar.gz
  download_dir=$(mktemp -d)
  trap 'rm -rf "$download_dir"' EXIT
  cd "$download_dir"
  curl -fLO "$release_url/$archive"
  curl -fLO "$release_url/$archive.sha256"
  shasum -a 256 -c "$archive.sha256"
  mkdir -p "$HOME/.local"
  tar -xzf "$archive" -C "$HOME/.local"
)
```

Start the CLI from a terminal.

```sh
"$HOME/.local/bin/codex-advisor"
```

In the CLI, use `/model` to select your main model. Enter `/advisor gpt-6.2-sol` if that model is available and differs from your main model. Use `/advisor` to check the setting and `/advisor off` to turn it off. The launcher starts a local server for this build, so you can keep your existing `codex` installation. [Advisor commands](./docs/slash_commands.md#advisor)

## OpenAI Codex

<p align="center"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/codex/blob/main/.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>
</br>
If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>codex app</code> or visit <a href="https://chatgpt.com/codex?app-landing-page=true">the Codex App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Upstream Codex quickstart

### Installing and running Codex CLI

Run the following on Mac or Linux to install Codex CLI:

```shell
curl -fsSL https://chatgpt.com/codex/install.sh | sh
```

Run the following on Windows to install Codex CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/codex/install.ps1 | iex"
```

The standalone installers download from `https://releases.openai.com/codex` by default and fall back to GitHub Releases if a metadata or asset download is unavailable. To force GitHub Releases, set `CODEX_INSTALLER_USE_RELEASES_OPENAI_COM` to `false` (`0` and `no` are also accepted):

```shell
curl -fsSL https://chatgpt.com/codex/install.sh | CODEX_INSTALLER_USE_RELEASES_OPENAI_COM=false sh
```

```powershell
$env:CODEX_INSTALLER_USE_RELEASES_OPENAI_COM='false'; irm https://chatgpt.com/codex/install.ps1 | iex
```

Codex CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask codex
```

Then simply run `codex` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codex-x86_64-unknown-linux-musl`), so you likely want to rename it to `codex` after extracting it.

</details>

### Using Codex with your ChatGPT plan

Run `codex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Codex as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codex-in-chatgpt).

You can also use Codex with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
