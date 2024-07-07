# Settlement Program

Central immutable settlement program on Solana for DeAI network.

## Network Entities

### Network Admin
- Initializing the network state
- Configuring the parameters and thresholds of the network
- Approval and granting of licenses to partners

### Network Partner
- Applying and paying to have access to the network
- Defining partnership terms and verification criteria for request sent
- Providing models to be used for requests
- Sending requests for generation to the network

### Node Operators
- Claiming partner requests
- Giving a request proposal
- Processing partner requests using partner provided models
- Receiving incentives for processing requests

## Instructions

### `InitializeSettlement`

Initialize Verifier accounts and set initial configuration

### `ConfigureSettlement`

Change Verifier configuration values.

### `RegisterNode`

Register Node into program to be eligible for participation incentives

### `VerifyOutput`

Verify that the output from the node is within acceptable standard and disburse incentives accordingly.

# State (Account PDAs)

**NetworkFacts**

**NetworkConfig**

**PartnerLicense**

**NodeRegistrationReceipt**
