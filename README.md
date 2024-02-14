# Introduction

This is a JSON RPC Client for SEQ without any of the dependencies of AvalancheGo, HyperSDK, or SEQ to ensure compatibility with rollups

## Overview

This project consists of three main parts:

1. Client: The functions that communicate with SEQ.
2. Types: The types defined for client.
3. Request: What sends the request to the server(SEQ).

Each part has its own README that you can check out for more details.

## Getting Started

To use this client, you need to run SEQ. You can do this at https://github.com/AnomalyFi/nodekit-seq. You will need to input your chainID, network ID, and uri as well as 2nd chain ID(rollup namespace) from there.