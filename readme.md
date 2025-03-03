<h1 align="center">Solana Acres</h1>
<p align="center">
<a href="https://x.com/solanaacres"><img src="https://img.shields.io/twitter/follow/solanaacres?style=social" height="100%" alt="Twitter Follow: solanaacres" /></a>
</p>

![small_banner](https://cdn.prod.website-files.com/6740d85c4e3daeef29a89470/67c5eb068e0b4435b4c46adf_994d1fee8b0d0f9ff00a95616423ddf1.png)

![banner_2400_800-export](https://cdn.prod.website-files.com/6740d85c4e3daeef29a89470/67c5ea976ecc177255bc9a5c_image%20(2)%20(3).gif)

The goal of this project is to create a decentralized and community-driven MetaVerse style game.

This repo includes the front-end game in which users can play and interact with the game on the Solana Network and offchain data.

Looking to help out? Read our [contributing docs](https://github.com/sovagpt/solanaacres)

By contributing you agree to our [terms and services](https://solana-acres.gitbook.io/solana-acres)

# Website | Official Links

### 🌻 Main Website

[https://www.solanaacres.xyz/](https://www.solanaacres.xyz/)

### 🧑‍🌾 Start Farming | Play

[https://www.solanaacres.xyz/play](https://www.solanaacres.xyz/play.html)

### 😕 Stuck somewhere ?

Read the official docs/ litepaper here: [docs.solanaacres.xyz](https://solana-acres.gitbook.io/solana-acres)

### 👶 How to play ?

Follow the official guide here: [How to Play?](https://solana-acres.gitbook.io/solana-acres)

### ❓ Need Help ?

First Read the FAQ's here: [FAQ's](https://solanaacres.freshdesk.com/support/solutions/101000195014)

Still not satisfied? Submit a support ticket here: [Submit a ticket](https://solanaacres.freshdesk.com/support/tickets/new)

### ‼️ Have a suggestion/proposal/cool idea ?

Please add a new idea for the community to vote on, here: [Solana Acres Idea Portal](https://solanaacres.ideas.aha.io/)

# 🎨 Sunnyside Assets

Solana Acres uses crops, icons and tiles from Daniel Diggle's SunnySide Asset Pack.

These raw assets/tiles are not in this repo. You must purchase the asset pack if you wish to extend these assets or use them elsewhere.

[Download Here](https://danieldiggle.itch.io/sunnyside)

# 👶 Getting Started

You can take a look at the instructions in [CODE_CONTRIBUTING.md](https://github.com/sovagpt/solanaacres) to get started on open-source contribution for Solana Acres

# 🧪 Testing

`yarn test`

This runs a range of business logic unit tests in the repo.

The plan is to use react testing library to test some of the core user interactions as well.

# ⚙️ Architecture

We use `xstate` to control the manage the user and session using a State Machine approach. This prevents our application from getting into invalid states and handles the use cases of switching accounts, networks, etc.

The primary states include:

- Connecting (connecting to MetaMask)
- Ready (Waiting for user input - Start)
- Signing (Sign a message to verify the account on the API)
- Authorising (Checking if a user has an account/farm)
- Unauthorised (when one of the above state transitions fails)
- Authorised (Play the game!)

# ⚙️ Vite

The app uses vite for bundling and development purposes. You can set build specific configuration in `vite.config.ts`

# 🌈 Tailwind

Tailwind is our CSS tool of choice. It enables us to:

- Use utility based classes
- Consistent theming (view `tailwind.config.js`)
- Perform CSS processing to minimize build sizes

# 🏷️ ERC1155 Metadata

Metadata is generated from markdown files.

Prerequisites:

`yarn global add ts-node`

To add new item:

1. Create `{SFT id}.md` file in `metadata\markdown` folder
2. Add `{SFT id}.png(gif)` file to `public\erc1155\images` folder
3. Run `yarn metadata`

# 🗃️ Directory Organization

- Assets: Images, Music, Branding and other Media
- Components: Reusable react components
- Lib: Utils, classes, machines and more.
- Features: Core domain concepts that have their own use cases/boundaries.
  Each feature (e.g. crops) has a similar nested structure of components, graphql & lib that are specific only to that feature.


# ⚖️ No Licence

The previous version was used unethically on other Blockchains. The team is working on deciding the licence that will best suit our community. Until then, the code falls under No Licence and cannot be reused.

All media assets (images and music) are not available for use in commercial or private projects.

To access the crops, resources and land tiles, please refer to the [SunnySide Asset Pack](https://danieldiggle.itch.io/sunnyside)

If you wish to use Bumpkin NFTs or custom Solana Acres collectibles in your own project please reach out to the core team on [Discord](https://discord.com/invite/solanaacres).
