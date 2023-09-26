#!/bin/bash

cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-unknown-linux-gnu
zip -r ./rg35xx-boxart-converter-x86-64-pc-windows-gnu.zip ./target/x86_64-pc-windows-gnu/release/rg35xx_boxart_converter.exe
zip -r ./rg35xx-boxart-converter-x86-64-unknown-linux-gnu.zip ./target/x86_64-unknown-linux-gnu/release/rg35xx_boxart_converter