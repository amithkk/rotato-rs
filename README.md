# Rotato - Windows Screen Rotation Utility

This originally started a little headless utility to enable hotkey driven screen rotation without depending on a specific graphics driver, circumventing the brightness reset bug on NVidia chipsets.

It then evolved into an experiment on integrating WinUI with Rust (Still a WIP)

## Usage

`cargo build` builds a windows executable, embedding the GUI resource as part of the binary. Utilize the "Resource Editor" (bundled with Visual Studio) to modify elements