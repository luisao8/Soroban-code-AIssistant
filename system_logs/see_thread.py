import os
from dotenv import load_dotenv
from openai import OpenAI


load_dotenv()

api_key = os.getenv("API_KEY")
assistant_id = os.getenv("ASSISTANT_ID")
client = OpenAI(api_key=api_key)


my_thread = client.beta.threads.retrieve("thread_abc123")
thread_messages = client.beta.threads.messages.list("thread_abc123")
print(my_thread)
print(thread_messages)

