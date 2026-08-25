import click
from traces_sm.client import client, console

@click.group(name="zkp")
def zkp_group():
    """ZKP commands"""
    pass

@zkp_group.command()
@click.option("--name", required=True)
@click.option("--output")
def prove_possession(name, output):
    console.print(f"[yellow]Placeholder for ZKP prove-possession of '{name}'[/yellow]")

@zkp_group.command()
@click.option("--name", required=True)
@click.option("--proof", required=True)
def verify_possession(name, proof):
    console.print(f"[yellow]Placeholder for ZKP verify-possession of '{name}'[/yellow]")

