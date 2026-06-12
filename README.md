# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
Here is the English version of your project template. You can copy and paste this directly into your submission or README.md file:

Title
SecondHand TrustGuard (Stellar Guarded Escrow)

Description
Currently, buyers and sellers of high-value second-hand items (such as laptops, motorcycles, or smartphones) in peer-to-peer (P2P) transactions constantly face the risk of scams. Buyers are afraid to transfer money upfront and receive faulty goods, while sellers fear handing over the item first only to be ghosted or have the item swapped.

SecondHand TrustGuard was created to completely solve this pain point. It is a dApp providing a decentralized escrow service using a Smart Contract on the Soroban (Stellar) platform. The system securely locks the buyer's funds (USDC/XLM) on-chain and only releases the payment to the seller when the transaction is successful. If an issue arises, a neutral Arbitrator (Guardian) will step in to resolve the dispute and ensure fairness.

Features
Secure Escrow: Locks USDC/XLM payments directly on-chain via the create_escrow function, ensuring neither party can arbitrarily withdraw the funds.

Dispute Resolution: Integrates an issue-reporting mechanism (raise_dispute) and empowers the Arbitrator (Guardian) to verify evidence. The Guardian can then call the resolve_dispute function to release funds to the rightful party.

Near-Zero Fees: Leveraging the Stellar/Soroban network, the transaction fee for each escrow contract is under $0.0001 — significantly cheaper than traditional Letter of Credit (L/C) fees or Ethereum gas fees.

Lightning Fast: The time to create an escrow and disburse funds is almost instantaneous (3-5 seconds based on Stellar's block consensus time), ensuring that real-world P2P trading remains seamless and uninterrupted.

Contract
Contract link:
https://stellar.expert/explorer/testnet/contract/CDSJTLQBINYSFE2EWJW6D5MKMVTRVIYSQCQPP65S7OTHCPT2QAXGTMDK
(Note: If you have re-deployed your contract and received a new ID, make sure to replace the CDSJ... ID in this link with your new one).

Contract's screenshot:
(Please take a screenshot of your contract on the Stellar Expert explorer using the link above, or a screenshot of your Terminal showing a successful invocation, and insert it here).

Future scopes
Web3 Frontend Development: Build an intuitive User Interface (UI) integrated with the Freighter wallet so users can interact easily without using the Command Line Interface (CLI).

Reputation System: Implement a trust-scoring system for Arbitrators (Guardians) based on their dispute resolution history to prevent bias and fraud.

Time-locked Escrow: Add a countdown timer feature. For example, if the buyer does not respond or confirm within 3 days, the contract will automatically release the funds to the seller.

Multi-token Support: Integrate Stellar's built-in DEX so the buyer can lock funds in XLM, while the seller receives USDC through an automatic swap mechanism.


CDSJTLQBINYSFE2EWJW6D5MKMVTRVIYSQCQPP65S7OTHCPT2QAXGTMDK
Profile
Name: Bang (You can update this with your full name if preferred)

Skills: Smart Contract Development, Rust, Soroban, Stellar Network.

GitHub: github.com/players06
