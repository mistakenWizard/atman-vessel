<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# › 1. I have 5 agy cli accounts, but agy cli allows only 1 logged in at a time, 1 codex cli account.

2. I need an orchestrator type framework, that can 1. use codex cli as the main thinker, 2. use agy cli / switch between different login for
my development workflow if I hit limits.
3. all of these are consumer facing subscription based flow. My pain point is just the manual auth switch whenever needed. and loss of context
during auth switch. I'm not a heavy user and most of the time don't hit usage limits, but will hit that occasionally and then things get stuck

Present / brainstorom on ideas/ implementation that can solve my problem, but won't violate the tnc of both gpt/ agy (and in future claude)
while keeping my workflow sane, and context intact.

Here's a full breakdown of the architecture and implementation path for your problem. The report covers everything end-to-end.

## The Short Answer

Your problem has two halves: **auth switching** and **context loss**. They need separate solutions that plug together. The key insight is: automate swapping *pre-authenticated credential files*, never automate the *OAuth login flow* — that's the ToS-safe boundary for both OpenAI and Google.

## The Core Architecture

**Layer 1 — Profile Isolation (Auth)**
Both CLIs support environment variable overrides for their config directory:[^1][^2]

- Codex → `CODEX_HOME=/path/to/profile`; auth stored in `auth.json`
- Antigravity → `GEMINI_CLI_HOME=/path/to/profile`; auth stored in `oauth_creds.json`[^3][^4]

You log in once per account using the official binary pointed at that directory. After that, switching = swapping an env var. No re-auth, no browser flow.

**Layer 2 — Context Preservation**
Antigravity CLI natively reads `.antigravity.md` files hierarchically (global → project → subdirectory) on every session start. Codex reads `AGENTS.md`. Keep a living `HANDOFF.md` (auto-generated via `npx handoff-md`) in your project root — it captures current task, recent git diffs, open blockers. Every new session starts with: *"Read HANDOFF.md first."*[^5][^6][^7]

**Layer 3 — Quota Detection**
Don't poll APIs. Watch the CLI's stderr/stdout for `"RESOURCE_EXHAUSTED"` (Antigravity) or `"ran into limit"` (Codex). A 50-line Bash or Node.js wrapper does this: detect the error → generate `HANDOFF.md` → switch to next Antigravity profile → relaunch with context.[^8][^9]

## Existing Tools You Can Use Today

- **`aisw`** — Rust binary, manages named profiles for Codex + Antigravity + Claude. `aisw use agy-2` switches in one command. Uses native auth flows, not proxies — ToS safe.[^10]
- **`fuelcheck`** — Shows remaining quota across all your CLI tools before you hit the wall.[^11]
- **`handoff-md`** — `npx handoff-md` generates a portable context file from your git repo.[^12][^5]


## ToS Guardrails

The Feb 2026 Antigravity 403 ban wave hit users of open-source wrappers that **re-implemented the auth handshake** (OpenClaw, etc.). Your approach avoids this entirely — you're using the official `agy` binary, just pointed at a different credential directory. Same for Codex; owning multiple subscriptions and switching between pre-authenticated files is personal use.[^13][^14]

The report above has the full implementation detail including the Bash failover script, profile directory structure, `.antigravity.md` context strategy, and a phased rollout plan.
<span style="display:none">[^15][^16][^17][^18][^19][^20][^21][^22][^23][^24][^25][^26][^27][^28][^29][^30][^31][^32][^33][^34][^35][^36][^37][^38][^39][^40][^41][^42][^43][^44][^45][^46][^47][^48][^49][^50][^51][^52][^53][^54][^55][^56][^57][^58][^59][^60][^61][^62][^63][^64][^65][^66][^67][^68][^69][^70][^71][^72][^73][^74][^75][^76][^77]</span>

<div align="center">⁂</div>

[^1]: https://geminicli.com/docs/get-started/authentication/

[^2]: https://discuss.ai.google.dev/t/urgent-mass-403-tos-bans-on-agy-api-antigravity-for-open-source-cli-users-paid-tier/124508

[^3]: https://www.reddit.com/r/ChatGPTCoding/comments/1mod6h2/codex_cli_rate_limiting/

[^4]: https://www.reddit.com/r/codex/comments/1shlox8/using_of_multiple_openai_subscriptions_for_codex/

[^5]: https://github.com/google-agy/agy-cli/issues/23796

[^6]: https://github.com/google-agy/agy-cli/issues/2815

[^7]: https://github.com/google-agy/agy-cli/issues/8248

[^8]: https://inventivehq.com/knowledge-base/agy/where-configuration-files-are-stored

[^9]: https://www.cometapi.com/how-to-change-agy-cli-directory/

[^10]: https://inventivehq.com/knowledge-base/openai/where-configuration-files-are-stored

[^11]: https://developers.openai.com/codex/config-advanced

[^12]: https://burakdede.com/blog/switch-accounts-claude-code-codex-agy-cli/

[^13]: https://github.com/Soju06/codex-lb

[^14]: https://www.reddit.com/r/codex/comments/1qh3asl/best_tools_to_load_balance_across_multiple_codex/

[^15]: https://www.reddit.com/r/opencodeCLI/comments/1s2lg0w/i_built_a_cli_to_check_all_my_ai_subscription/

[^16]: https://www.reddit.com/r/ClaudeAI/comments/1rgbt2d/i_built_a_cli_tool_with_claude_code_that/

[^17]: https://dev.to/lakshmisravyavedantham/i-built-a-cli-that-gives-any-ai-instant-context-about-your-project-3ig8

[^18]: https://news.ycombinator.com/item?id=47183826

[^19]: https://geminicli.com/docs/cli/agy-md/

[^20]: https://addyosmani.com/blog/agy-cli/

[^21]: https://github.com/openai/codex/issues/2448

[^22]: https://aws.amazon.com/blogs/opensource/introducing-cli-agent-orchestrator-transforming-developer-cli-tools-into-a-multi-agent-powerhouse/

[^23]: https://www.reddit.com/r/AI_Agents/comments/1rpoutg/how_i_built_a_multiagent_orchestration_platform/

[^24]: https://mariozechner.at/posts/2025-11-30-pi-coding-agent/

[^25]: https://github.com/google-agy/agy-cli/issues/3565

[^26]: https://github.com/agy-cli-extensions/workspace/issues/173

[^27]: https://www.reddit.com/r/googlecloud/comments/1puzgdi/configuring_gemini_cli_authentication_properly/

[^28]: https://developer.onecodex.com/api-reference/rate-limits

[^29]: https://google-agy.github.io/agy-cli/docs/get-started/authentication.html

[^30]: https://apidog.com/blog/codex-usage-limits/

[^31]: https://milvus.io/ai-quick-reference/how-do-i-switch-models-in-agy-cli

[^32]: https://dev.to/vishalmysore/cli-based-ai-agent-tool-calling-with-cli-5059

[^33]: https://www.mintlify.com/asharali-codefied/Worklog-manager-agy/configuration/agy-setup

[^34]: https://geminicli.com/docs/reference/configuration/

[^35]: https://mintlify.com/google-agy/agy-cli/reference/environment-variables

[^36]: https://google-agy.github.io/agy-cli/docs/get-started/configuration.html

[^37]: https://stackoverflow.com/questions/593334/how-to-use-multiple-aws-accounts-from-the-command-line

[^38]: https://www.truefoundry.com/docs/ai-gateway/agy-cli

[^39]: https://www.youtube.com/watch?v=00zyMF0P4Pk

[^40]: https://github.com/Kilo-Org/kilocode/issues/2076

[^41]: https://www.linkedin.com/posts/iromin_gemini-cli-tutorial-seriespart-3-configuration-activity-7346495185725317120-Weo2

[^42]: https://www.reddit.com/r/ClaudeAI/comments/1rov69v/i_built_a_cli_tool_to_manage_multiple_claude_code/

[^43]: https://geminicli.com/docs/resources/tos-privacy/

[^44]: https://ai.google.dev/agy-api/terms

[^45]: https://github.com/google-agy/agy-cli/discussions/3642

[^46]: https://google-agy.github.io/agy-cli/docs/tos-privacy.html

[^47]: https://geminicli.com/docs/resources/quota-and-pricing/

[^48]: https://community.openai.com/t/codexcli-is-anyone-using-multiple-openai-logins-projects-at-the-same-time/1265832

[^49]: https://developers.google.com/agy-code-assist/resources/faqs

[^50]: https://developers.google.com/agy-code-assist/resources/quotas

[^51]: https://milvus.io/ai-quick-reference/how-do-i-authenticate-and-connect-codex-cli-to-my-openai-account

[^52]: https://geminicli.com/docs/resources/faq/

[^53]: https://github.com/openai/codex/issues/12029

[^54]: https://github.com/router-for-me/CLIProxyAPI/issues/1382

[^55]: https://community.openai.com/t/codex-bug-multiple-github-accounts/1263218

[^56]: https://github.com/anomalyco/opencode/issues/18391

[^57]: https://github.com/google-agy/agy-cli/issues/6170

[^58]: https://github.com/openai/codex/issues/4432

[^59]: https://github.com/topics/load-balancer?l=python

[^60]: https://openai.github.io/openai-agents-python/tools/

[^61]: https://ai.google.dev/agy-api/docs/oauth

[^62]: https://marketplace.visualstudio.com/items?itemName=woozy-masta.codex-switch

[^63]: https://www.philschmid.de/agy-cli-cheatsheet

[^64]: https://blog.laozhang.ai/en/posts/agy-cli-install

[^65]: https://community.openai.com/t/login-with-openai-api-key-environment-variable/1371740

[^66]: https://geminicli.com/docs/tools/mcp-server/

[^67]: https://github.com/openai/codex/issues/14330

[^68]: https://developers.openai.com/codex/auth/ci-cd-auth

[^69]: https://github.com/google-agy/agy-cli

[^70]: https://www.mintlify.com/google-agy/agy-cli/reference/environment-variables

[^71]: https://codelabs.developers.google.com/agy-cli-hands-on

[^72]: https://geminicli.com/docs/cli/settings/

[^73]: https://www.youtube.com/watch?v=oLbMCI393lI

[^74]: https://github.com/google-agy/agy-cli/discussions/22041

[^75]: https://github.com/google-agy/agy-cli/issues/22288

[^76]: https://awesomeclaude.ai/claude-code-tips

[^77]: https://dometrain.com/blog/creating-the-perfect-claudemd-for-claude-code/

