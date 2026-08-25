from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    ENCLAVE_URL: str = "https://localhost:8443"
    ENCLAVE_TLS_VERIFY: bool = False
    DATABASE_URL: str = "sqlite:///./sm-metadata.db"
    DATABASE_CIPHER_KEY: str = "00000000000000000000000000000000"
    API_HOST: str = "0.0.0.0"
    API_PORT: int = 8080
    LOG_LEVEL: str = "INFO"
    JWT_SECRET: str = "supersecretjwtkey"
    ADMIN_TOKEN: str = "bootstrap-admin-token"

    class Config:
        env_file = ".env"

settings = Settings()
