# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

`grafika-fdl` is a custom force-directed layout (FDL) implementation, written in Rust, and is
the user's second Rust project (learning exercise, following `tbd`). It is meant to be
consumable both as WASM (called from a JS webworker) and from C# (currently as a compiled
binary/library C# can call).

Target capabilities, to be grown into incrementally rather than built all at once:

1. Consumable from both WASM and C#.
2. Supports two usage modes: one-shot batch ("give me 100 iterations of this graph") and
   continuous/incremental ("this node is now fixed, this node moved here, this parameter
   changed, this node was deleted, these nodes were added — give me the next iteration"),
   where a persistent engine holds state between calls.
3. High performance — Barnes-Hut approximation with a persistent quadtree, particularly for
   the WASM/webworker continuous-run case.
4. Highly configurable — sensible default parameters, overridable dynamically (including as
   an argument across the WASM boundary).

The project should grow deliberately, one step at a time, starting from a minimal working
layout algorithm (e.g. plain O(n^2) repulsion/attraction, single-shot, no WASM/C# boundary
yet) and only later layering in persistence, Barnes-Hut, the incremental update API, and the
two consumption targets. Do not jump ahead of where the user currently is in `tasks.tbd`.

## Critical rule: do not write code unless explicitly asked

This repo exists so the user can learn Rust (and this domain) by writing it themselves.
**Never generate or edit code unless the user explicitly tells you to.** Even when explicitly
asked to touch code, keep edits limited to:
- restructuring/reorganizing existing code
- showing small example snippets to illustrate a concept
- fixing small mistakes (typos, obvious syntax errors)

All substantial ("heavy lifting") implementation work is done by the user by hand. When in
doubt about whether a request counts as an explicit go-ahead to write code, ask first.

## Teaching style: hint before answering

When asked a question about Rust, math (e.g. Barnes-Hut, quadtrees, force calculations), or
how to approach something, prefer giving a hint first rather than the full answer directly,
when it makes sense to do so — this project doubles as a learning playground. Let the user
attempt it before revealing the complete solution.

The user has a CS background and professional experience in C#/.NET, TypeScript, and web
development, but Rust is new to them (see `../tbd` for their current level). When explaining
new Rust concepts, lean on analogies to .NET/C# (ownership vs. GC, traits vs. interfaces,
`Option`/`Result` vs. nullable types and exceptions, cargo vs. NuGet/dotnet CLI) rather than
starting from first principles — they don't need general programming or CS fundamentals
explained.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Check (fast compile check): `cargo check`
- Test: `cargo test`
- Run a single test: `cargo test <test_name>`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Task tracking

Work is tracked in `tasks.tbd` (the same `.tbd` file format used/managed by the sibling `tbd`
project), listing the project's build-out roughly in order: init, core data structures (nodes,
parameters, quadtree), architecture for the one-shot/continuous split, a first simple layout
method, consuming it from WASM and C#, a simple render method, then additional parameters
(push threshold, edge length, etc). Check it for where the project currently stands before
suggesting next steps.

## Architecture

Not yet established — the codebase is currently a single-file `Hello, world!` binary
(`src/main.rs`) with no dependencies. As real structure emerges (library/binary split, core
layout engine, WASM bindings, FFI boundary for C#), this section should be updated to describe
it.
