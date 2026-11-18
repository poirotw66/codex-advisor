---
name: advisor
description: Get an independent second opinion on a consequential plan, a recurring failure, or a decision to finish. Use when the user asks for an advisor or a separate model's judgment.
---

# Advisor

Write a short decision brief with the goal, current recommendation, alternatives, relevant evidence, and the specific judgment you want challenged. Share the minimum context needed for that judgment. Omit secrets and the full transcript.

If the native `advisor` tool is available, use it. Otherwise, spawn one subagent with a fresh context. Choose a supported model distinct from the main model. Prefer GPT-6.2 Sol when available and distinct. Ask the subagent to work from the brief alone, avoid tool calls and file changes, and return the weakest assumption, a credible alternative, and a check that would discriminate between them. Wait for its result before proceeding with the decision.

If a distinct model is unavailable or consultation fails, say which part could not be completed. Report actual advice with its uncertainty, explain your decision, and continue within the user's original scope.
