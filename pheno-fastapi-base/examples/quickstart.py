"""Quickstart: pheno-fastapi-base

Run with::

    uvicorn examples.quickstart:app --reload

Then visit:
  http://127.0.0.1:8000/healthz
  http://127.0.0.1:8000/items/foo
"""

from pheno_fastapi_base import AppError, create_app

app = create_app(title="quickstart", version="0.1.0")


@app.get("/items/{item_id}")
async def get_item(item_id: str) -> dict:
    if item_id == "missing":
        raise AppError("NotFound", f"no item {item_id!r}")
    return {"id": item_id, "name": f"item-{item_id}"}