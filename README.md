[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
<h1 align="center">Litemint Auction Contract</h1>
  <p align="center">
   Auction smart contract for the Litemint marketplace on Soroban, implementing timed auctions with support for both ascending and descending price mechanisms.
  </p>
</div>

## Note

_The contract code was uploaded to this public repo for the [Pre-Soroban Mainnet Testing & Feedback Week](https://dashboard.communityfund.stellar.org/scfevents/pre-soroban-mainnet-testing-feedback-week/instructionssubmission/suggestion/447) however please note that the official repos for **_Litemint smart contracts_** are hosted by [@Litemint](https://github.com/litemint) Github—will be publicly available with Soroban mainnet release._


<!-- ABOUT THE PROJECT -->
## About Litemint Auction Contract

Since 2021, the Litemint marketplace has utilized the Stellar DEX for time-based auctions, leveraging time-bound, pre-auth transactions [details in our blog](https://blog.litemint.com/anatomy-of-a-stellar-powered-auction-on-litemint/). While these auctions offer security and interoperability, they lack flexibilities, such as anti-snipe mechanisms and varied bidding strategies like descending auctions.

The Litemint Auction Contract on [Soroban](https://soroban.stellar.org) (Stellar's Rust-based smart contracts platform), addresses these limitations. The smart contract enhances the Litemint marketplace while co-existing with our SDEX-based method, offering users a comprehensive and versatile auction experience.

This contract implements a range of features, including:

- [X] Time-based auctions with decentralized resolution.
- [X] Descending price auctions (see [behavior_descending_price.rs](https://github.com/FredericRezeau/litemint-auction-contract/blob/main/src/auctions/behavior_descending_price.rs)) supporting linear or compound discount, and customizable frequency/rate.
- [X] Ascending price auctions (see [behavior_ascending_price.rs](https://github.com/FredericRezeau/litemint-auction-contract/blob/main/src/auctions/behavior_ascending_price.rs)) with "**_buy now_**" option.
- [X] Support for `reserve price` and `ask price`.
- [X] Rust Traits-based behavior for easy auction types extension (search impl for `resolve` and `calculate_price` for examples).
- [X] Anti-snipe mechanism. Auction sniping automatically increases the auction duration (time configurable by admin) and prevents the sniper to either cancel or submit a new bid.
- [X] Configurable marketplace commission rate.
- [X] Extendable auction duration by seller.
- [X] Support for concurrent and cancellable bids.

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

* Rust min 1.71 and Soroban
* Update Rust for macOS, Linux, or another Unix-like OS
*  ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ``` 

  Set up your environment for smart contract development with Soroban on Rust by following the instructions provided in the link below:
  [https://soroban.stellar.org/docs/getting-started/setup](https://soroban.stellar.org/docs/getting-started/setup)

### Running tests and building

1. Cloning the Repository:
   ```sh
   git clone https://github.com/FredericRezeau/litemint-auction-contract.git
   ```
2. cd into reop, if you opended in VC editor go striaght to step 3 if not find repo in your path.
  ```sh
   Yoursysuser $ or % //yourpath/eg.desktop/yourgituserifforked/litemint-auction-contract.git
   ```

3. Running Tests:
   ```sh
   cargo test -- --nocapture
   ```
4. Building the Contract:
   ```sh
   cargo build --target wasm32-unknown-unknown --release
   ```

<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Fred Kyung-jin Rezeau - [@FredericRezeau](https://twitter.com/fredericrezeau)

Litemint Marketplace: [https://litemint.com](https://litemint.com)

Join our discord server: [https://litemint.gg](https://litemint.gg)


<!-- MARKDOWN LINKS & IMAGES -->
[license-shield]: https://img.shields.io/github/license/FredericRezeau/soroban-snooker.svg?style=for-the-badge
[license-url]: https://github.com/FredericRezeau/soroban-snooker/blob/master/LICENSE
[twitter-shield]: https://img.shields.io/badge/-Twitter-black.svg?style=for-the-badge&logo=twitter&colorB=555
[twitter-url]: https://twitter.com/fredericrezeau

[rust-shield]: https://img.shields.io/badge/Rust-000000?style=flat-square&logo=Rust&logoColor=white
[rust-url]: https://www.rust-lang.org
[javascript-shield]: https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black
[javascript-url]: https://vanilla-js.com
