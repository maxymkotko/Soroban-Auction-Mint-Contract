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


<!-- ABOUT THE PROJECT -->
## About Litemint Auction Contract

Since 2021, the Litemint marketplace has utilized the Stellar DEX for time-based auctions, leveraging time-bound, pre-auth transactions [details in our blog](https://blog.litemint.com/anatomy-of-a-stellar-powered-auction-on-litemint/). While these auctions offer security and interoperability, they lack flexibilities, such as anti-snipe mechanisms, varied bidding strategies like descending auctions, "Buy now" for ascending auctions and so on. The Litemint auction contract, for the Rust-based smart contracts platform [Soroban](https://soroban.stellar.org), addresses these limitations. It enhances the Litemint marketplace while co-existing with our existing SDEX-based implementation, offering users a comprehensive and versatile auction experience.

This contract offers a range of features, including:

- [X] Time-based auctions.
- [X] [Ascending price auctions](https://github.com/FredericRezeau/litemint-auction-contract/blob/main/src/auctions/behavior_ascending_price.rs) with "buy now" option.
- [X] [Descending price auctions](https://github.com/FredericRezeau/litemint-auction-contract/blob/main/src/auctions/behavior_descending_price.rs) supporting linear or compound discount, and customizable frequency/rate.
- [X] Anti-snipe mechanism.
- [X] Configurable marketplace commission rate.
- [X] Easily extendable bidding behavior using Rust Traits.
- [X] Extendable auction duration.
- [X] Support for concurrent and cancellable bids.

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

* Rust and Soroban

  Set up your environment for smart contract development with Soroban on Rust by following the instructions provided in the link below:
  [https://soroban.stellar.org/docs/getting-started/setup](https://soroban.stellar.org/docs/getting-started/setup)

### Running tests and building

1. Cloning the Repository:
   ```sh
   git clone https://github.com/FredericRezeau/litemint-auction-contract.git
   ```
2. Running Tests:
   ```sh
   cargo test -- --nocapture
   ```
3. Building the Contract:
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
