#!/usr/bin/env python3
# ^ picks this is up in git bash on windows, nice

import typer
import subprocess
import shlex
from typing_extensions import Annotated
from typing import Optional

# Docs
# https://typer.tiangolo.com/
# https://rust-book.cs.brown.edu/appendix-04-useful-development-tools.html
# https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html

def sh(cmd: str) -> None:
    """ Helper for running terminal commands """
    subprocess.run(shlex.split(cmd))

# Create app
app = typer.Typer()

@app.command()
def build(self) -> None:
    """ Build cargo project """
    self.sh("cargo build")

@app.command()
def build(mode: Annotated[Optional[str], typer.Argument()] = "d") -> None:
    """ Build the cargo project and produce an executable """
    if mode == "r": 
        sh("cargo build --release")
    else:
        sh("cargo build")
        
@app.command()
def run() -> None:
    """ Build (if files changed) and run cargo project """
    sh("cargo run")

@app.command()
def check() -> None:
    """ Check to see code compiles (does not produce executable) """
    sh("cargo check")

@app.command()
def fmt() -> None:
    """ Format rust code """
    sh("cargo fmt")

@app.command()
def fix() -> None:
    """ Automatically fix compiler warnings and manage rust editions """
    sh("cargo fix")

@app.command()
def lint() -> None:
    """ Lint rust code """
    sh("cargo clippy")

if __name__ == "__main__":
    app()