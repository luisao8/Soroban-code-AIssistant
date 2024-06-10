from openai import OpenAI
from dotenv import load_dotenv
import os

load_dotenv()


api_key = os.getenv("API_KEY")
client = OpenAI(api_key=api_key)
assistant_id = os.getenv("ASSISTANT_ID")


vector_store = client.beta.vector_stores.create(name="Soroban Kowledge")

print(vector_store)

