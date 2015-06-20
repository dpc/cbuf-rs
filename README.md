[![Build Status](https://travis-ci.org/dpc/cbuf-rs.svg?branch=master)](https://travis-ci.org/dpc/cbuf-rs)

# cbuf

## Introduction

Non-thread-shareable, simple and efficient Circular Buffer
implementation that can store N elements when full (typical circular
buffer implementations store N-1) without using separate flags.

Uses only `core` so can be used in `#[no_std]` projects by using
`no_std` feature.

## Usage

In `Carto.toml`

	[dependencies]
	dpc-cbuf = "*"

In `src/main.rs`:

	extern crate cbuf;
