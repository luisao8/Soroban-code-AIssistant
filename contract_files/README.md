# Crowdfunding Campaign Smart Contract

This repository contains the source code for a crowdfunding campaign smart contract built on the Soroban platform. The contract allows users to contribute to a campaign, request refunds if the campaign is unsuccessful, and disburse funds if the campaign is successful.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [Contribution](#contribution)
  - [Finalization](#finalization)
  - [Disbursement](#disbursement)
  - [Refund](#refund)
- [Contract Structure](#contract-structure)
- [Modules](#modules)
- [Contributing](#contributing)
- [License](#license)

## Overview

This smart contract is designed to manage crowdfunding campaigns on the Soroban platform. It enables users to contribute funds, finalize the campaign, disburse funds if successful, and request refunds if unsuccessful. The contract uses Stellar's native asset (XLM) for all transactions.

## Features

- **Contribute**: Users can contribute XLM to the campaign.
- **Finalize**: The admin can finalize the campaign after the deadline.
- **Disburse**: If the campaign is successful, the admin can disburse the funds to the recipient.
- **Refund**: If the campaign is unsuccessful, contributors can request refunds.

## Getting Started

### Prerequisites

- Rust programming language (version 1.73 or later)
- Cargo package manager
- Soroban SDK

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-repo/crowdfunding-campaign.git
   cd crowdfunding-campaign

## Usage

### Contribution
To contribute to the campaign, users can call the `contribute` function with their address and the amount of XLM they wish to contribute.

### Finalization
The admin can finalize the campaign after the deadline by calling the `finalize` function. This sets the campaign status based on the total contributions.

### Disbursement
If the campaign is successful, the admin can disburse the funds to the recipient by calling the `disburse` function.

### Refund
If the campaign is unsuccessful, contributors can request a refund by calling the `refund` function. This transfers their contributions back to them.

## Contract Structure
The contract is structured into several modules, each responsible for specific functionality:

- `lib.rs`: The main entry point of the contract, defining the contract structure and implementing the primary functions.
- `contribution.rs`: Handles contributions to the campaign.
- `finalization.rs`: Finalizes the campaign.
- `disbursement.rs`: Disburses funds if the campaign is successful.
- `refund.rs`: Processes refunds if the campaign is unsuccessful.
- `security.rs`: Provides security checks.
- `data.rs`: Defines data structures and keys used in the contract.

## Modules

### `lib.rs`
The main entry point of the contract, defining the `CrowdfundingContract` structure and implementing the primary functions such as `contribute`, `finalize`, `disburse`, and `refund`.

### `contribution.rs`
Handles user contributions. Users can contribute XLM to the campaign, and their contributions are recorded.

### `finalization.rs`
Allows the admin to finalize the campaign after the deadline. It checks if the campaign has met its goal and updates the status accordingly.

### `disbursement.rs`
If the campaign is successful, the admin can disburse the funds to the recipient.

### `refund.rs`
Handles refund requests from contributors if the campaign is unsuccessful.

### `security.rs`
Provides security functions, such as requiring admin authorization.

### `data.rs`
Defines the data structures and keys used in the contract, such as contributions, total contributions, status, and deadlines.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request with your improvements.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
