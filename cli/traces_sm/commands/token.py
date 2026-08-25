import click
from traces_sm.client import client, console

@click.group(name="token")
def token_group():
    """Manage tokens"""
    pass

@token_group.command()
@click.option("--subject", required=True)
@click.option("--scopes", multiple=True)
@click.option("--ttl", type=int, default=3600)
def create(subject, scopes, ttl):
    data = {"subject": subject, "scopes": list(scopes), "ttl": ttl}
    result = client.post("/v1/tokens", json=data)
    console.print_json(data=result)

@token_group.command()
@click.option("--id", "token_id", required=True)
def revoke(token_id):
    result = client.delete(f"/v1/tokens/{token_id}")
    console.print(f"[green]Successfully revoked token '{token_id}'[/green]")

