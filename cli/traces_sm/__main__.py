import click
from traces_sm.commands import secret, key, token, attest, zkp, lifecycle, dkg, entropy

@click.group()
def cli():
    """Traces-SM: NIST SP 800-57 Compliant SGX Secrets Manager CLI"""
    pass

cli.add_command(secret.secret_group, name="secret")
cli.add_command(key.key_group, name="key")
cli.add_command(token.token_group, name="token")
cli.add_command(attest.attest_group, name="attest")
cli.add_command(zkp.zkp_group, name="zkp")
cli.add_command(lifecycle.lifecycle_group, name="lifecycle")
cli.add_command(dkg.dkg_group, name="dkg")
cli.add_command(entropy.entropy_group, name="entropy")

if __name__ == "__main__":
    cli()
