# Smart Contract Development Assistant

![image](https://github.com/luisao8/Soroban-code-AIssistant/assets/74673031/21132e03-a200-4fc1-b922-3a426dbd957d)

## Overview

This project is an initial proof of concept designed to assist in the creation and iterative improvement of smart contracts using OpenAI's Assistants API. The primary focus is on the Stellar blockchain and Soroban smart contracts. The workflow involves gathering user input, generating initial designs, refining with expert input, building the smart contract, and finalizing the documentation. Currently, the project uses dummy conversational input for demonstration purposes.

## Process Description

1. **Environment Setup**: 
   - Load necessary libraries, environment variables, and initialize the OpenAI client.

2. **Helper Functions**:
   - **Thread Management**: Create threads to organize messages.
   - **Run Management**: Create runs, poll their status, and return results when completed.
   - **File Management**: Save content to specified files and clean AI responses by removing malformed comments.

3. **Workflow Phases**:
   - **Understanding the Problem**: Gather user inputs and frame the problem using dummy conversational input.
   - **Initial Design and Iteration**: Generate an initial design and iterate with expert input.
   - **Building the Smart Contract**: Extract file names and build the necessary files for the smart contract.
   - **Finalizing and Documenting**: Generate the final smart contract, the associated TOML file, and documentation.

4. **Main Program Execution**:
   - Execute the entire workflow from understanding the problem to finalizing and documenting the smart contract.

## Instructions on How to Use

1. **Setup Environment**:
   - Ensure you have your OpenAI API key and Assistant ID set up in a `.env` file.

2. **Run the Script**:
   - Execute the script to start the workflow. It will create threads, generate runs, and manage messages to build and refine the smart contract iteratively.

3. **Review Generated Files**:
   - After the script completes, review the generated smart contract files in the `contract_files` directory. This includes the main contract file, the TOML file, and the README documentation.

## Notes

- This project is an initial proof of concept and requires further refinement for production use.
- An example smart contract generated by this process is included in the `examples` folder for reference.

## Example

To run the script, follow these steps:

1. Clone the repository.
2. Set up your environment variables in a `.env` file.
3. Run the `main` function in the script.
4. Review the output files in the `contract_files` directory.

This proof of concept aims to streamline the smart contract development process, leveraging AI to enhance efficiency and accuracy. Further improvements and refinements will be made to ensure robustness and security in the generated contracts.

---

By following the above instructions, you can use this assistant to generate and refine smart contracts, making the development process more efficient and reliable.
