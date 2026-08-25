import click
from rich.console import Console
from rich.table import Table
from traces_sm.client import SmClient

console = Console()

@click.group(name="lifecycle")
def lifecycle_group():
    """NIST SP 800-57 Key Lifecycle Management"""
    pass

@lifecycle_group.command("transition")
@click.option("--id", "key_id", required=True, help="Key ID to transition")
@click.option("--state", "target_state", type=click.Choice(["PRE_OPERATIONAL", "OPERATIONAL", "DEACTIVATED", "EXPIRED", "REVOKED", "DESTROYED"]), required=True, help="Target NIST state")
@click.option("--reason", help="Transition or revocation reason")
def transition(key_id, target_state, reason):
    """Transition a key through NIST SP 800-57 lifecycle states"""
    client = SmClient()
    res = client.post("/v1/lifecycle/transition", json={"key_id": key_id, "target_state": target_state, "reason": reason})
    if res.get("success"):
        data = res["data"]
        console.print(f"[bold green]Key state transitioned successfully![/bold green]")
        console.print(f"Key ID: [cyan]{data.get('key_id')}[/cyan]")
        console.print(f"New State: [yellow]{data.get('new_state')}[/yellow]")
    else:
        console.print(f"[bold red]Transition failed:[/bold red] {res.get('error')}")

@lifecycle_group.command("shred")
@click.option("--id", "key_id", required=True, help="Key ID to crypto-shred")
def shred(key_id):
    """NIST SP 800-88 Cryptographic Erasure (Crypto-Shredding)"""
    if not click.confirm(f"Are you sure you want to permanently crypto-shred key {key_id}? This is irreversible."):
        return
    client = SmClient()
    res = client.post("/v1/lifecycle/shred", json={"key_id": key_id, "confirmation": key_id})
    if res.get("success"):
        console.print(f"[bold red]Key crypto-shredded and storage overwritten (NIST SP 800-88)[/bold red]")
    else:
        console.print(f"[bold red]Crypto-shredding failed:[/bold red] {res.get('error')}")
