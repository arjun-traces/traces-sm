import click
from rich.console import Console
from traces_sm.client import SmClient

console = Console()

@click.group(name="entropy")
def entropy_group():
    """NIST SP 800-90A/B/C Entropy & DRBG Status"""
    pass

@entropy_group.command("health")
def health():
    """Check NIST SP 800-90B DRBG Health Tests (APT & RCT)"""
    client = SmClient()
    res = client.get("/v1/entropy/health")
    if res.get("success"):
        data = res["data"]
        console.print(f"[bold cyan]NIST SP 800-90B DRBG Health Status:[/bold cyan]")
        rct = "[green]PASSED[/green]" if data.get("rct_passed") else "[red]FAILED[/red]"
        apt = "[green]PASSED[/green]" if data.get("apt_passed") else "[red]FAILED[/red]"
        console.print(f"  Repetition Count Test (RCT): {rct}")
        console.print(f"  Adaptive Proportion Test (APT): {apt}")
        console.print(f"  Reseed Counter: [yellow]{data.get('reseed_count')}[/yellow]")
        console.print(f"  Entropy Source: [bold white]{data.get('source')}[/bold white]")
    else:
        console.print(f"[bold red]Entropy check failed:[/bold red] {res.get('error')}")
