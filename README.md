# Donut.bingo

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)


Repository made for smart contract / program for Donut.bingo web game

![alt text](https://github.com/9bit-club/donut.bingo/blob/main/unrelated/banner.png?raw=true)


## Instructions

- bingo() Used for players to burn their collection and receive 9bit membership
- fill() - [ADMIN ONLY] - Used by admin to add rules to blockchain which BB + Donuts are required. It also uploads 9bit membership NFT to PDA


## Dependencies 

| Rust Package | Version |
| ------ | ------ |
| solana-program | "1.10.30" |
| anchor-lang | "0.25.0" |
| anchor-spl | "0.25.0" |
| spl-token | "3.3.0" |


