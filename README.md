# AI-Assisted Soroban Smart Contract Generation System

![image](https://user-images.githubusercontent.com/your-username/unique-image-id.png](https://github.com/luisao8/Soroban-code-AIssistant/assets/74673031/21132e03-a200-4fc1-b922-3a426dbd957d)


## Introduction

This project aims to build an AI-assisted system to generate Soroban smart contracts efficiently. The system accelerates smart contract creation and conceptualization, making the process more efficient for developers experienced in building Soroban smart contracts.

## Project Overview

### Main Objective

Develop a smart contract development assistant that gathers user information and uses artificial intelligence to generate and validate contracts. This assistant provides a robust starting point for developers, ensuring high-quality and safer contracts.

### Project Phases

1. **Initial Pilot**:
   - Create a functional version of the assistant that can gather information, generate smart contracts, validate and package them, and allow testing on a testnet.

2. **SaaS Web Application**:
   - Develop a standalone web platform where developers can use the assistant, generate, validate, and download smart contracts for review and adjustment.

3. **Future Enhancements**:
   - Scale the solution for non-technical users, integrating new features and improvements based on user feedback and ecosystem growth.

## Technology and Functionality

### User Interface

- A user-friendly web interface to capture user requirements.

### AI for Code Generation

- Use multiple specialized AI agents to:
  - Process and understand user inputs.
  - Generate modular sections of the smart contract in JSON.
  - Assemble sections into a complete contract.
  - Validate and review the generated code.

### Product Delivery

- Generate a zip file containing the contract and auxiliary files.
- Provide a secure link for downloading and testing on the testnet.

## Detailed Workflow

### Part 1: Preparing the Smart Contract

1. **User Input Gathering AI**:
   - Collect all necessary inputs from the user for creating the smart contract.
   - Engage in a dynamic conversation to ensure all required information is covered.

2. **Problem Proposer AI**:
   - Summarize user inputs into a coherent problem statement.
   - Outline the type of smart contract needed.

3. **Initial Design AI**:
   - Propose a full initial structure for the smart contract.
   - Outline the modules and sections needed.

4. **Specialist AIs for Contract Refinement**:
   - Provide expert input on specific areas to improve the contract's depth and quality.
   - Focus areas include Horizon API, RPC providers, oracles, cross-contract interactions, and security.

5. **Iterative Improvement and Final Design**:
   - Incorporate suggestions from all specialists.
   - Create a detailed design based on iterative improvements.

### Part 2: Building the Smart Contract

6. **Builder AI**:
   - Build each block of the smart contract sequentially based on the final design.
   - Ensure coherence and consistency of variables and logic.

7. **Final Review AI**:
   - Review the completed smart contract for accuracy and security.

8. **Documentation AI**:
   - Generate detailed documentation based on the final design and built contract.
   - Include descriptions of modules, functions, purposes, usage examples, and guidelines.

9. **Compilation and Packaging**:
   - Compile and package the final smart contract into a zip file for developer testing.

## Technologies Used

- **Web-app**: Bubble
- **Backend**: Google Cloud Functions
- **AI Integration**: OpenAI API for natural language processing and code generation
- **Blockchain**: Compatibility with Soroban

## Example Project Structure

