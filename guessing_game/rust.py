#!/usr/bin/env python3
# ^ picks this is up in git bash on windows, nice

import typer
import subprocess
import shlex
from typing_extensions import Annotated
from typing import Optional
import os
import shutil

# Docs
# https://typer.tiangolo.com/
# https://rust-book.cs.brown.edu/appendix-04-useful-development-tools.html
# https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html
# https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html

def sh(cmd: str) -> None:
    """ Helper for running terminal commands """
    subprocess.run(shlex.split(cmd))

# Create app
app = typer.Typer()

################################################
#### RUST BUILD AND RUN COMMANDS
################################################

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


################################################
#### RUST CODE TOOLS
################################################
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

@app.command()
def gen_docs() -> None:
    """ Generate docs for all dependencies and open in the browser. """
    sh("cargo doc --open")

################################################
#### RUST CARGO PROJECT COMMANDS
################################################
@app.command()
def update() -> None:
    """ Update cargo crates. Ignores current cargo.lock and then generates a new one when done. """
    sh("cargo update")


################################################
#### EXTRAS
################################################

@app.command()
def update_cli() -> None:

    # crude hacky check to prevent run in cargo projects
    if "template" not in os.getcwd() or "rust.py" not in os.listdir():
        exit("Called from wrong directory")

    # get project top level dir
    project_root = f"{os.getcwd()}\\.."

    # for each dir in project top level
    for dir in os.listdir(project_root):

        # check if it is a cargo project
        if(os.path.isfile(f"{project_root}\{dir}\Cargo.toml")):
            
            # Update/create CLI file
            shutil.copyfile(
                f"{project_root}\\template\\rust.py", 
                f"{project_root}\\{dir}\\rust.py"
            )

if __name__ == "__main__":
    app()