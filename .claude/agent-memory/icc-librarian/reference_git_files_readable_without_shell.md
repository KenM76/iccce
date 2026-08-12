---
name: iccce-git-files-readable-without-shell
description: A shell-less librarian CAN corroborate commit hashes, push events and dates — .git/logs/HEAD, .git/logs/refs/remotes/*, .git/config and .git/refs/* are plain text and readable with the Read tool
metadata:
  type: reference
---

**`.git/` holds plain-text files that the Read tool opens.** Reading one
is **not** running `git`, and it does not violate "this agent has no
shell". Discovered 2026-08-12 at the Pass 6 + Pass 7 filing, after
eleven filings in which every commit hash was carried as *reported*.

**Where to look, and what each answers:**

| Path | Answers |
|---|---|
| `.git/config` | the configured remote name and **URL** |
| `.git/logs/HEAD` | **every HEAD advance**: old hash, new hash, author, **epoch seconds + tz offset**, and the operation (`commit`, `commit (initial)`, `reset`, `rebase`, `checkout`, `merge`) with the **commit subject line** |
| `.git/logs/refs/remotes/<remote>/<branch>` | **push/fetch events** — a line reading `update by push` with a timestamp. A left-hand side of all zeros means **the remote branch did not exist before that line** ⇒ that line is the first publish |
| `.git/refs/heads/<branch>` · `.git/refs/remotes/<remote>/<branch>` | the current tips; equal tips ⇒ nothing local is unpushed |
| `.git/packed-refs` | may not exist in a young repo — absence is not an error |

**What it establishes, and the three limits that matter:**

- ✅ A commit **exists** with that hash and subject line; **when** it
  landed; whether history was **rewritten** (any `reset`/`rebase`/amend
  line); whether a **push** happened and when.
- ❌ **NOT the commit's CONTENTS.** No reflog reading ever supplies
  that. Keep saying "contents unverified".
- ❌ **NOT a repository's visibility.** A push to a *private* repo
  produces an identical reflog. Public/private is a server-side setting
  no file records — it stays the operator's report.
- ❌ **NOT reliably a commit count.** The reflog counts HEAD advances in
  *this clone*; it implies the branch's commit count only if nothing was
  authored elsewhere and fetched and nothing was pruned. If it disagrees
  with a dispatch, **record the discrepancy, assert neither number.**

**Epoch → local date:** `2026-01-01 00:00 UTC = 1767225600`;
`2026-08-11 00:00 UTC = 1786406400`. Subtract, divide by 86400, then
apply the offset printed on the line (e.g. `-0400`).

**Why this is worth keeping.** The first time it was used it caught
three things a dispatch had wrong: **the filing's DATE** (dispatch
2026-08-11, reflog 2026-08-12), **a commit hash carried in three project
documents** (`edcb60e` → `edce48b`), and **a commit count** (49 vs 45).
It is the cheapest verification available to an agent with no shell, and
it should be the first thing done whenever a dispatch names commits.

Related: [[iccce-pass-status]], [[iccce-verify-own-draft-too]].
