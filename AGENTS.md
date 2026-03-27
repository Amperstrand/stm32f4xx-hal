# stm32f4xx-hal

Hardware abstraction layer for STM32F4 series microcontrollers. Fork of stm32-rs/stm32f4xx-hal with additions for DSI, LTDC, and SDRAM peripherals on the STM32F469.

## Build

```bash
cargo build --features stm32f469
cargo build --features stm32f469,dsihost,defmt
```

## Architecture

This is a fork of the upstream [stm32-rs/stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal). It adds support for peripherals not yet in upstream:

- `dsi.rs` — DSI host controller
- `ltdc.rs` — LCD-TFT display controller
- `fmc.rs` — Flexible memory controller (SDRAM)

All other peripherals (GPIO, USART, I2C, SPI, USB OTG, etc.) are from upstream.

## Upstream Relationship

This is a **fork** of stm32-rs/stm32f4xx-hal. Changes to existing upstream peripherals should be proposed upstream first. Only Amperstrand-specific additions (DSI, LTDC, SDRAM) live here.

## Upstream Interaction Policy

**NEVER file PRs or issues on upstream projects (stm32-rs, embassy-rs, etc.) without human review and approval.** AI-generated bug diagnoses can be confidently wrong. If you find a potential upstream bug:
1. Document your findings in an Amperstrand repo issue first
2. Include all evidence (register dumps, test results, methodology)
3. Let a human decide whether to escalate

Since this IS a fork of an upstream project, this policy is especially important — we don't want to file issues against our own upstream based on AI hallucinations.

See [Amperstrand/micronuts#19](https://github.com/Amperstrand/micronuts/issues/19) for a retrospective on how a confident misdiagnosis wasted upstream maintainer time.
