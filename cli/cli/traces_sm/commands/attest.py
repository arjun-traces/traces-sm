import click
from traces_sm.client import client, console

@click.group(name="attest")
def attest_group():
    """Attestation commands"""
    pass

@attest_group.command()
@click.option("--output")
def quote(output):
    result = client.post("/v1/attest/quote", json={})
    if output:
        with open(output, "w") as f:
            f.write(result.get("quote", ""))
    console.print("[green]Quote retrieved successfully[/green]")
    if not output:
        console.print_json(data=result)

@attest_group.command()
@click.option("--quote", "quote_file", required=True)
@click.option("--mrenclave")
@click.option("--mrsigner")
def verify(quote_file, mrenclave, mrsigner):
    with open(quote_file, "r") as f:
        quote_data = f.read()
    
    data = {"quote": quote_data, "mrenclave": mrenclave, "mrsigner": mrsigner}
    result = client.post("/v1/attest/verify", json=data)
    console.print_json(data=result)

