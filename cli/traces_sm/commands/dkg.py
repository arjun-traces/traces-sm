import click
from rich.console import Console
from rich.table import Table
from traces_sm.client import SmClient

console = Console()

@click.group(name="dkg")
def dkg_group():
    """Distributed Key Generation & Node Topology"""
    pass

@dkg_group.command("setup")
@click.option("--threshold-m", default=2, help="Threshold M required shares")
@click.option("--total-n", default=3, help="Total N shares")
@click.option("--nodes", help="Comma-separated node endpoints")
def setup(threshold_m, total_n, nodes):
    """Configure distributed M-of-N threshold DKG topology"""
    node_list = nodes.split(",") if nodes else ["https://localhost:8443"]
    client = SmClient()
    res = client.post("/v1/dkg/setup", json={"threshold_m": threshold_m, "total_n": total_n, "nodes": node_list})
    if res.get("success"):
        console.print(f"[bold green]DKG Topology Configured ({threshold_m}-of-{total_n})[/bold green]")
    else:
        console.print(f"[bold red]DKG Setup Failed:[/bold red] {res.get('error')}")

@dkg_group.command("nodes")
def list_nodes():
    """List current DKG topology nodes"""
    client = SmClient()
    res = client.get("/v1/dkg/nodes")
    if res.get("success"):
        table = Table(title="DKG Node Topology")
        table.add_column("Node ID", style="cyan")
        table.add_column("Endpoint", style="white")
        table.add_column("Role", style="yellow")
        table.add_column("Status", style="green")
        for node in res["data"]:
            table.add_row(node["id"], node["endpoint"], node["node_role"], node["status"])
        console.print(table)
    else:
        console.print(f"[bold red]Failed to fetch nodes:[/bold red] {res.get('error')}")
