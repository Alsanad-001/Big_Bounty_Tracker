🛡️ Stellar Bug Bounty Tracker
A decentralized, transparent, and secure bug bounty management system built on the Stellar network using Soroban smart contracts.

📖 Project Description
The Bug Bounty Tracker is designed to bridge the gap between security researchers (white-hat hackers) and organizations. By leveraging the Stellar blockchain, the platform ensures that every bounty posted is backed by an immutable record. It eliminates the "black box" nature of traditional bounty programs, providing a public ledger of open vulnerabilities and verified resolutions.

## ⚙️ What it does

* **On-Chain Ledger:** Serves as a transparent, immutable record for all security vulnerabilities and bounty programs.
* **Bounty Initialization:** Allows organizations to officially register a bug bounty by locking in the description and the reward amount on the blockchain.
* **Trustless Hunting:** Enables security researchers to hunt for bugs with the guarantee that the bounty terms are fixed and verifiable on-chain.
* **Verified Resolution:** Once a bug is fixed, the organization interacts with the contract to officially mark it as "Resolved."
* **Proof of Achievement:** Automatically records the solver’s wallet address upon resolution, providing permanent, on-chain credit to the researcher.

## 🚀 Features

* **On-Chain Identity:** Uses Stellar `Address` types to ensure all participants are verified via their cryptographic keys.
* **Secure Authorization:** Built-in `require_auth()` checks ensure that only the person who created the bounty has the power to resolve it.
* **Immutable Bounty Ledger:** Once a bounty is created, its parameters and status are stored in Soroban's persistent storage, making it tamper-proof.
* **Status Tracking:** A clear state machine logic (`Open` vs. `Resolved`) prevents double-claiming or confusion over bounty availability.
* **Reputation Building:** Since the solver's address is stored on-chain upon resolution, it serves as a permanent, verifiable resume for security researchers.
* **Soroban Native:** Optimized for the Stellar network, ensuring high performance and low transaction fees.

### 🛠️ Technical Details
* **Platform:** Soroban (Stellar Smart Contracts)
* **Language:** Rust

**Contract Functions:**
* **`create_bounty`**: Initializes a new bug report.
* **`resolve_bounty`**: Marks a bug as fixed and assigns credit to the solver.
* **`get_bounty`**: Fetches the current state and details of any bounty ID.

# Deploys smart contract 

Contract address : CDTFPUNWABRHBNWAATH3Q2XFBONYUKMJWXVUASJ4OL4WPPDMRAPLE7R5

<img width="1308" height="659" alt="image" src="https://github.com/user-attachments/assets/92cc306b-788f-491a-b967-7c74a35f1ccc" />
