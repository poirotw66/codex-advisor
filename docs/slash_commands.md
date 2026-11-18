# Slash commands

For an overview of Codex CLI slash commands, see [this documentation](https://developers.openai.com/codex/cli/slash-commands).
## Advisor

Use `/advisor <model-id>` to set the second-opinion model for subsequent turns. Choose a different model from the current provider's `/model` list. Use `/advisor` to check the selection and `/advisor off` to disable it. The command saves `advisor_model` in user configuration. Higher-precedence configuration still applies.

For example, start with GPT-6 Luna and enter `/advisor gpt-6.2-sol` if both are available. Then ask:

> Compare the two implementation approaches. Ask the advisor to challenge the assumption that most affects your recommendation before you edit files.

Once enabled, the main model can also consult the advisor on its own at consequential decisions, after recurring failures, or before finishing complex work. Each consultation sends only the main model's decision brief through the thread's model provider and uses additional tokens. The advisor cannot use tools or approve actions. The main model owns decisions and execution.

To enable the advisor for a single CLI invocation, pass `-c 'advisor_model="gpt-6.2-sol"'` when launching Codex.
