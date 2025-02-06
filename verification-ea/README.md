# verification-ea

This directory hosts the proof verification using Chainlink's external adaptor. Chainlink External Adaptors are how you integrate custom off chain computations into your on-chain application.

The project is built using `Rocket` and `Nova` crates. The general workflow of invoking the EA is illustrated in the figure bellow. 

![Chainlinks EA Workflow](external-adapter.svg)

The Verifier smart contract sends requests to Chainlink nodes through the Operator contract, custom computations are performed by leveraging external adapters.
