#!/usr/bin/env python3

import typer
import subprocess
import shlex

# Docs
# https://typer.tiangolo.com/

# Create typer app
app = typer.Typer()

@app.command()
def build():
    subprocess.run(shlex.split(f"rustc main.rs"))

@app.command()
def run():
    subprocess.run(shlex.split(f"./main.exe"))

if __name__ == "__main__":
    app()