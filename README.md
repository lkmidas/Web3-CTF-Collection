# Web3-CTF-Collection

| **CTF**       | **Challenge**           | **Chain** | **Platform** | **Bug/Solution**                                    |
|---------------|-------------------------|-----------|--------------|-----------------------------------------------------|
| N1CTF 2022    | Utility Payment Service | Solana    | Vanilla Rust | Integer underflow                                   |
| N1CTF 2022    | Simple Staking          | Solana    | Anchor       | PDA seed collision                                  |
| idek CTF 2022 | Blockchain 1            | Solana    | Anchor       | Missing signer authorization                        |
| idek CTF 2022 | Blockchain 2            | Solana    | Anchor       | Integer underflow                                   |
| idek CTF 2022 | Blockchain 3            | Solana    | Anchor       | Missing check leading to Admin re-initialization    |
| Dice CTF 2023 | Baby Solana             | Solana    | Anchor       | Erroneous Rust code leading to constraint bypassing & Unchecked negative signed integers |
| Dice CTF 2023 | Otterworld              | Solana    | Anchor       | Creating an account whose Pubkey starts with "osec" |
| LACTF 2023    | breakup                 | Ethereum  | Solidity     | Missing checks & `public` attribute on the `burn` method allowing any user to burn any NFT |
| Angstrom 2023 | Sailor                  | Solana    | Vanilla Rust | Account confusion between `SailorUnion` and `Registration` |
| Sekai CTF 2023 | The Bidding            | Solana    | Anchor       | Creating a fake account whose PDA collides with another account to be initialized by other user, locking them out of the bidding process |