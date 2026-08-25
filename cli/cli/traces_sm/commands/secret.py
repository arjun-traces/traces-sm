import click
from traces_sm.client import client, console

@click.group(name="secret")
def secret_group():
    """Manage secrets"""
    pass

@secret_group.command()
@click.option("--name", required=True)
@click.option("--value", required=True)
@click.option("--type", "secret_type", default="generic")
@click.option("--ttl", type=int, default=0)
@click.option("--tags")
def create(name, value, secret_type, ttl, tags):
    tags_dict = {}
    if tags:
        for tag in tags.split(","):
            k, v = tag.split("=")
            tags_dict[k] = v
    data = {"name": name, "value": value, "secret_type": secret_type, "ttl": ttl, "tags": tags_dict}
    result = client.post("/v1/secrets", json=data)
    console.print(f"[green]Successfully created secret '{name}'[/green]")

@secret_group.command()
@click.option("--name", required=True)
@click.option("--output-format", default="json")
def get(name, output_format):
    result = client.get(f"/v1/secrets/{name}")
    if output_format == "raw":
        print(result.get("value", ""))
    else:
        console.print_json(data=result)

@secret_group.command()
@click.option("--name", required=True)
@click.option("--value", required=True)
def update(name, value):
    result = client.put(f"/v1/secrets/{name}", json={"value": value})
    console.print(f"[green]Successfully updated secret '{name}'[/green]")

@secret_group.command()
@click.option("--name", required=True)
@click.option("--force", is_flag=True)
def delete(name, force):
    result = client.delete(f"/v1/secrets/{name}")
    console.print(f"[green]Successfully deleted secret '{name}'[/green]")

@secret_group.command()
@click.option("--owner")
@click.option("--type", "secret_type")
@click.option("--format", "output_format", default="table")
def list(owner, secret_type, output_format):
    result = client.get("/v1/secrets")
    console.print_json(data=result)

