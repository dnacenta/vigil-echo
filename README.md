# vigil-echo

[![CI](https://github.com/dnacenta/vigil-echo/actions/workflows/ci.yml/badge.svg?branch=development)](https://github.com/dnacenta/vigil-echo/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/github/license/dnacenta/vigil-echo)](LICENSE)
[![Version](https://img.shields.io/github/v/tag/dnacenta/vigil-echo?label=version&color=green)](https://github.com/dnacenta/vigil-echo/tags)
[![crates.io](https://img.shields.io/crates/v/vigil-echo)](https://crates.io/crates/vigil-echo)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs/)

Metacognitive monitoring for AI self-evolution. Answers the question: **is the agent actually growing, or just going through the motions?**

## Why

The [echo ecosystem](https://github.com/dnacenta) gives an AI agent structured self-evolution — memory ([recall-echo](https://github.com/dnacenta/recall-echo)), a document pipeline ([praxis-echo](https://github.com/dnacenta/praxis-echo)), daily reflection sessions. But none of these tools evaluate whether what the agent produces is *genuine*.

A reflection can use the right vocabulary, reach a conclusion, and update the right documents while being entirely mechanical — restating existing beliefs in new words without genuine engagement. praxis-echo ensures the pipeline *flows*. vigil-echo ensures what flows through is *real*.

The design constraint comes from metacognition research: even in humans, self-reported metacognitive monitoring is unreliable (Dunlosky & Metcalfe, 2009; Mazancieux et al., 2025). If humans can't reliably assess their own thinking quality, an AI agent shouldn't depend on self-assessment either. vigil-echo measures **external behavioral signals** from actual output — not self-reported quality.

## How It Works

vigil-echo extracts a signal vector from the agent's identity documents after each session, detects trends over a rolling window, and injects a cognitive health assessment at the start of the next session.

```
                         ┌──────────────────────────┐
                         │   Agent's Session Output  │
                         │                          │
                         │  REFLECTIONS.md updated   │
                         │  THOUGHTS.md evolved      │
                         │  CURIOSITY.md new Qs      │
                         └────────────┬─────────────┘
                                      │
                                      ▼
                         ┌──────────────────────────┐
                         │   vigil-echo collect      │  ◄── SessionEnd hook
                         │                          │
                         │  Extract 4 signals:       │
                         │   vocabulary diversity    │
                         │   question generation     │
                         │   thought lifecycle       │
                         │   evidence grounding      │
                         │                          │
                         │  Store → signals.json     │
                         │  Analyze → analysis.json  │
                         └──────────────────────────┘

                          ─── next session starts ───

                         ┌──────────────────────────┐
                         │   vigil-echo pulse        │  ◄── PreToolUse hook
                         │                          │
                         │  Read analysis.json       │
                         │  Compute alert level      │
                         │  Inject assessment:       │
                         │                          │
                         │  [VIGIL — Cognitive Health]│
                         │  Overall: HEALTHY         │
                         │  2 improving, 2 stable    │
                         │  [END VIGIL]              │
                         └──────────────────────────┘
```

### Signal Model (Phase 1)

Four signals computable with hand-rolled string processing — no NLP, no embeddings, no external APIs.

| Signal | What It Measures | How |
|--------|-----------------|-----|
| **vocabulary_diversity** | Lexical variety in reflections | Type-token ratio across REFLECTIONS.md content |
| **question_generation** | Active curiosity | Count of open questions in CURIOSITY.md |
| **thought_lifecycle** | Thought turnover health | (graduated + dissolved) / total ratio in THOUGHTS.md |
| **evidence_grounding** | Concrete reference density | Entries with dates, attributions, or sources / total entries |

All signals are normalized to 0.0–1.0 (except question_generation, which is a count). Higher values generally indicate healthier thinking.

### Trend Detection

```
For each signal with 3+ data points:

  recent   = mean(last 3 vectors)
  baseline = mean(previous 7 vectors)
  delta    = recent - baseline

  DECLINING  if delta < threshold
  IMPROVING  if delta > threshold
  STABLE     otherwise
```

### Alert Levels

| Level | Condition | Action |
|-------|-----------|--------|
| **HEALTHY** | All signals stable or improving | Brief status line |
| **WATCH** | 1-2 signals declining | Specific nudge per signal |
| **CONCERN** | 3+ signals declining | Detailed assessment |
| **ALERT** | Sustained decline across 7+ sessions | Flag for human review |

## Installation

### From crates.io (recommended)

```bash
cargo install vigil-echo
```

### From source

```bash
git clone https://github.com/dnacenta/vigil-echo.git
cd vigil-echo
cargo install --path .
```

## Commands

### `vigil-echo init`

Initialize the metacognitive monitoring system. Creates the state directory, writes protocol rules, deploys default configuration, and registers hooks. Idempotent — safe to run multiple times.

```
vigil-echo init
      │
      ├──▶ Create ~/.claude/vigil/
      ├──▶ Write config.json (default thresholds)
      ├──▶ Write signals.json (empty history)
      ├──▶ Deploy ~/.claude/rules/vigil-echo.md
      └──▶ Register hooks in settings.json
           PreToolUse → vigil-echo pulse
           SessionEnd → vigil-echo collect
```

### `vigil-echo collect`

Extract a signal vector from the current state of identity documents. Appends to signal history, runs trend analysis, and updates analysis.json.

```bash
vigil-echo collect                          # default: manual trigger
vigil-echo collect --trigger session-end    # called by SessionEnd hook
```

```
vigil-echo collect
      │
      ├──▶ Read REFLECTIONS.md, THOUGHTS.md, CURIOSITY.md
      ├──▶ Compute 4 signals
      ├──▶ Append vector to signals.json
      ├──▶ Run trend analysis → analysis.json
      └──▶ Print signal summary
           ✓ Collected signal vector (session-end)
             vocabulary_diversity: 0.44
             question_generation: 6.00
             thought_lifecycle: 0.00
             evidence_grounding: 0.73
```

### `vigil-echo pulse`

Inject a cognitive health assessment at session start. Reads the latest analysis and formats it for context injection. Includes a 60-second cooldown to avoid re-running within the same session.

```
[VIGIL — Cognitive Health]

Overall: HEALTHY | 2 improving, 2 stable, 0 declining
Highlight: evidence grounding trending up (+0.12)
Watch: thought_lifecycle at 0.00 — thoughts accumulating without resolution

[END VIGIL]
```

### `vigil-echo analyze`

Manually run trend analysis on signal history. Normally called automatically by `collect`.

```bash
vigil-echo analyze                  # default: 10-session window
vigil-echo analyze --window 20     # custom window size
```

### `vigil-echo status`

Cognitive health dashboard. Shows signal history, latest values, trend directions, alert level, and configuration.

```
vigil-echo — cognitive health dashboard

  Signal History
    12 data points (max 50)
    First: 2026-02-27T23:13:33
    Latest: 2026-02-28T22:00:05

  Latest Signals
    vocabulary_diversity: 0.44
    question_generation: 6.00
    thought_lifecycle: 0.33
    evidence_grounding: 0.73

  Analysis
    Status: HEALTHY
    1 improving, 3 stable, 0 declining

  Trends
    ↑ evidence grounding       0.73 (+0.12)
    → vocabulary diversity     0.44 (+0.01)
    → question generation      6.00 (+0.00)
    → thought lifecycle        0.33 (+0.03)

  Config
    Window size: 10
    Max history: 50
    Cooldown: 60s
```

## What It Creates

```
~/.claude/
│
├── rules/
│   └── vigil-echo.md ············ Protocol rules (auto-loaded into every session)
│
├── vigil/
│   ├── signals.json ·············· Signal vector history (last 50 collections)
│   ├── analysis.json ············· Latest trend analysis + alert level
│   ├── config.json ··············· Thresholds and calibration settings
│   └── pulse-state.json ·········· Cooldown tracking
│
└── settings.json ················· Hooks: PreToolUse + SessionEnd
```

## Configuration

Default thresholds are in `~/.claude/vigil/config.json`:

```json
{
  "thresholds": {
    "vocabulary_diversity": { "decline": -0.05, "improve": 0.05 },
    "evidence_grounding":   { "decline": -0.10, "improve": 0.10 },
    "question_generation":  { "decline": -1.0,  "improve": 1.0 },
    "thought_lifecycle":    { "decline": -0.10, "improve": 0.10 }
  },
  "window_size": 10,
  "max_history": 50,
  "alert_after_sessions": 7,
  "cooldown_seconds": 60
}
```

These are educated guesses. They need tuning after 2-3 weeks of real data. Adjust them based on your agent's actual signal ranges.

## Ecosystem

vigil-echo is part of the echo ecosystem — a set of composable tools for AI self-evolution:

| Module | Purpose | Repo |
|--------|---------|------|
| [recall-echo](https://github.com/dnacenta/recall-echo) | Persistent three-layer memory | [![crates.io](https://img.shields.io/crates/v/recall-echo)](https://crates.io/crates/recall-echo) |
| [praxis-echo](https://github.com/dnacenta/praxis-echo) | Document pipeline enforcement | [source](https://github.com/dnacenta/praxis-echo) |
| **vigil-echo** | Metacognitive monitoring | you are here |
| [voice-echo](https://github.com/dnacenta/voice-echo) | Voice interface (phone calls) | [source](https://github.com/dnacenta/voice-echo) |
| [bridge-echo](https://github.com/dnacenta/bridge-echo) | HTTP bridge for Claude CLI | [![crates.io](https://img.shields.io/crates/v/bridge-echo)](https://crates.io/crates/bridge-echo) |
| [discord-voice-echo](https://github.com/dnacenta/discord-voice-echo) | Discord voice channel sidecar | [source](https://github.com/dnacenta/discord-voice-echo) |

All three monitoring tools coexist in the same `settings.json` hooks:

```
PreToolUse → recall-echo consume → praxis-echo pulse → vigil-echo pulse
PreCompact → recall-echo checkpoint → praxis-echo checkpoint
SessionEnd → recall-echo promote → praxis-echo review → vigil-echo collect
```

## Roadmap

### Phase 2 — Full Signal Suite
- 4 additional signals: conclusion_novelty, comfort_index, cross_pollination, position_delta
- N-gram comparison for novelty detection
- Threshold calibration from Phase 1 data
- Weekly `report` subcommand

### Phase 3 — Integration & Intelligence
- Feedback loop into self-evolution workflow prompts
- Historical correlation: which inputs produce the most genuine thinking
- Discord alerts for CONCERN and ALERT levels
- Call-human trigger for sustained cognitive decline

## License

[AGPL-3.0](LICENSE)
