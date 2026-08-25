import httpx
import yaml
import os
from rich.console import Console

console = Console()

class SmClient:
    def __init__(self):
        self.config_path = os.path.expanduser("~/.sm/config.yaml")
        self.host_url = "http://localhost:8080"
        self.token = "bootstrap-admin-token"
        self._load_config()

        self.client = httpx.Client(base_url=self.host_url, headers={"Authorization": f"Bearer {self.token}"})

    def _load_config(self):
        if os.path.exists(self.config_path):
            with open(self.config_path, "r") as f:
                config = yaml.safe_load(f) or {}
                self.host_url = config.get("host_url", self.host_url)
                self.token = config.get("token", self.token)

    def get(self, path: str):
        resp = self.client.get(path)
        return self._handle_response(resp)

    def post(self, path: str, json: dict = None):
        resp = self.client.post(path, json=json)
        return self._handle_response(resp)

    def put(self, path: str, json: dict = None):
        resp = self.client.put(path, json=json)
        return self._handle_response(resp)

    def delete(self, path: str):
        resp = self.client.delete(path)
        return self._handle_response(resp)

    def _handle_response(self, response):
        if response.status_code >= 400:
            console.print(f"[red]Error {response.status_code}:[/red] {response.text}")
            exit(1)
        return response.json()

client = SmClient()
