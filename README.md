<!--
  ====== Welcome ======
  Paste this into your profile’s README.md for a modern, interactive showcase.
-->

<!-- Hero Section -->
<div align="center">
  <h1>📝 Soldit</h1>
  <p><em>Decentralized Discussion Board (Reddit‑style) on Solana</em></p>

  <!-- Badges -->
  <p>
    <a href="https://github.com/parmeet20/soldit_contract/stargazers">
      <img src="https://img.shields.io/github/stars/parmeet20/soldit_contract?style=social" alt="Stars" />
    </a>
    <a href="https://github.com/parmeet20/soldit_contract/network/members">
      <img src="https://img.shields.io/github/forks/parmeet20/soldit_contract?style=social" alt="Forks" />
    </a>
    <a href="https://github.com/parmeet20/soldit_contract/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/parmeet20/soldit_contract" alt="License" />
    </a>
    <a href="https://github.com/parmeet20/soldit_contract/issues">
      <img src="https://img.shields.io/github/issues/parmeet20/soldit_contract" alt="Issues" />
    </a>
    <a href="https://crates.io/crates/anchor-lang">
      <img src="https://img.shields.io/crates/v/anchor-lang" alt="Anchor Version" />
    </a>
  </p>

  <!-- Demo GIF / Screenshot -->
  <p>
    <img src="https://raw.githubusercontent.com/parmeet20/soldit_contract/main/docs/demo.gif" alt="Soldit Demo" width="80%" />
  </p>
</div>

---

## 🚀 Why Soldit?

<p align="center">
  Built on Solana’s high‑throughput, low‑fee network, Soldit brings decentralized community discussions on‑chain—trustless voting, immutable posts, and user‑driven moderation.
</p>

---

## ✨ Core Features

<details>
  <summary><strong>👤 User Initialization</strong> <em>(On‑Chain Identity)</em></summary>
  <br>
  • **Initialize User Account**: Derive a unique PDA for each user.  
  • **Profile Metadata**: Store username, avatar URI, join timestamp.  
  • **Secure Ownership**: Only the wallet owner can update their profile.
</details>

<details>
  <summary><strong>🆕 Thread Management</strong> <em>(Post & Edit Topics)</em></summary>
  <br>
  • **Create Thread**: Mint a new on‑chain post with title, body, tags.  
  • **Update Thread**: Allow authors to edit content and metadata.  
  • **Immutable History**: Preserve previous versions via event logs.
</details>

<details>
  <summary><strong>💬 Commenting System</strong> <em>(Nested Replies)</em></summary>
  <br>
  • **Create Comment**: Post replies to any thread.  
  • **Update Comment**: Edit your own comments for clarity.  
  • **Threaded View**: Maintain parent–child relationships on‑chain.
</details>

<details>
  <summary><strong>👍 Voting Mechanism</strong> <em>(Upvotes & Downvotes)</em></summary>
  <br>
  • **Upvote Thread**: Increase thread visibility with upvotes.  
  • **Vote Comment**: Cast up/down votes on comments.  
  • **Anti‑Spam**: Single‑vote enforcement via PDAs.
</details>

---

## 📂 Code Highlights

| 📄 File                                                                                                           | 🔍 Description                                      |
| ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| [initialize_user.rs](programs/soldit/src/instructions/initialize_user.rs)  | Set up user profile PDA & metadata                  |
| [create_thread.rs](programs/soldit/src/instructions/create_thread.rs)            | Mint a new discussion thread                       |
| [update_thread.rs](programs/soldit/src/instructions/update_thread.rs)            | Edit thread content & tags                         |
| [create_comment.rs](programs/soldit/src/instructions/create_comment.rs)          | Post a comment reply to a thread                   |
| [update_comment.rs](programs/soldit/src/instructions/update_comment.rs)          | Modify existing comment content                    |
| [upvote_thread.rs](programs/soldit/src/instructions/upvote_thread.rs)            | Increment thread upvotes                           |
| [vote_comment.rs](programs/soldit/src/instructions/vote_comment.rs)              | Upvote or downvote a comment                       |
| [mod.rs](programs/soldit/src/instructions/mod.rs)                                  | Re‑exports all instruction handlers                |

---

## 🛠️ Getting Started

```bash
# 1. Clone the repo
git clone https://github.com/parmeet20/soldit_contract.git

# 2. Enter project directory
cd soldit_contract

# 3. Build & Deploy (requires Solana CLI + Anchor)
anchor build
anchor deploy
