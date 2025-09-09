# Infer Chain

**Decentralized AI Inference, Settled on Solana.**

Infer Chain is a DePIN protocol that connects users needing AI inference with a distributed network of compute providers, using the Solana blockchain as a robust settlement and coordination layer.

##  Why Infer Chain?

The demand for AI compute is exploding. Centralized providers lead to high costs, vendor lock-in, and single points of failure. Infer Chain solves this by:

*   **Reducing Costs:** A competitive marketplace for compute power.
*   **Enabling Earners:** Monetize your idle GPUs/TPUs by joining the network.
*   **Ensuring Censorship Resistance:** Open access to AI inference for everyone.
*   **Providing Verifiability:** cryptographically prove that work was done correctly.

##  How It Works

1.  **User submits a task** (e.g., an inference job) with payment to the Solana program.
2.  **Network nodes** (providers) listen for available tasks.
3.  **A node executes the task** and submits a proof of work.
4.  **The Solana program verifies the payment** and distributes rewards to the node.
5.  **Results are delivered** back to the user.

##  Architecture
┌─────────────┐ ┌──────────────┐ ┌──────────────┐
│ Client │ ◄──┤ Solana CLI │ ◄──┤ Rust SDK │
│ (Rust/CLI) │ │ & Library │ │ │
└─────────────┘ └──────────────┘ └──────────────┘
│ │ │
└────────────────────┼──────────────────┘
▼
┌──────────────┐
│ Solana │
│ Program │
│ (Smart Contract) │
└──────────────┘
│
▼
┌──────────────┐
│ Infer Node │
│ Network │
└──────────────┘
