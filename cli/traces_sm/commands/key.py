import click
from traces_sm.client import client, console

@click.group(name="key")
def key_group():
    """Manage keys"""
    pass

@key_group.command()
@click.option("--name", required=True)
@click.option("--algorithm", required=True)
@click.option("--tags")
def generate(name, algorithm, tags):
    tags_dict = {}
    if tags:
        for tag in tags.split(","):
            k, v = tag.split("=")
            tags_dict[k] = v
    data = {"name": name, "algorithm": algorithm, "tags": tags_dict}
    result = client.post("/v1/keys/generate", json=data)
    console.print(f"[green]Successfully generated key '{name}'[/green]")

@key_group.command()
@click.option("--name", required=True)
@click.option("--format", default="pem")
def public(name, format):
    result = client.get(f"/v1/keys/{name}/public?format={format}")
    console.print(result)

@key_group.command()
@click.option("--name", required=True)
@click.option("--message", required=True)
@click.option("--format", default="base64")
def sign(name, message, format):
    result = client.post(f"/v1/keys/{name}/sign", json={"message": message, "format": format})
    console.print_json(data=result)

@key_group.command()
@click.option("--name", required=True)
@click.option("--message", required=True)
@click.option("--signature", required=True)
def verify(name, message, signature):
    result = client.post(f"/v1/keys/{name}/verify", json={"message": message, "signature": signature})
    console.print_json(data=result)

