# cbuf - Circular buffer

<p align="center">
  <a href="https://travis-ci.org/dpc/cbuf-rs">
      <img src="https://img.shields.io/travis/dpc/cbuf/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://crates.io/crates/cbuf">
      <img src="http://meritbadge.herokuapp.com/cbuf?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/dpc">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/cbuf-rs/">Documentation</a></strong>
</p>

## Introduction

Non-thread-shareable, simple and efficient Circular Buffer implementation that
can store N elements when full (typical circular buffer implementations store
N-1) without using additional flags.

Uses only `core` so can be used in `#[no_std]` projects by using
`no_std` feature.

## Usage

In `Carto.toml`

	[dependencies]
	cbuf = "*"

In `src/main.rs`:

	extern crate cbuf;
