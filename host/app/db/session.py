from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from app.config import settings

# Wait, since pysqlcipher3 is used, the URI needs to be sqlite+pysqlcipher
# But we can fallback to standard sqlite for simple mock testing
db_url = settings.DATABASE_URL
if "pysqlcipher" in db_url:
    connect_args = {"pragmas": {"key": settings.DATABASE_CIPHER_KEY}}
    engine = create_engine(db_url, connect_args=connect_args)
else:
    engine = create_engine(db_url, connect_args={"check_same_thread": False})

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
