from openai import OpenAI
from dotenv import load_dotenv
import os
from instructions import designer_ai_prompt

load_dotenv()


api_key = os.getenv("API_KEY")
client = OpenAI(api_key=api_key)
assistant_id = os.getenv("ASSISTANT_ID")
instructions = os.getenv("INSTRUCTIONS")

name = "Smart Contract Development Assistant"
id = assistant_id 

assistant = client.beta.assistants.update(
  id,
  instructions=designer_ai_prompt,
  name=name,
  model="gpt-4o",
  temperature=0,
  tools=[{"type": "file_search"},{"type": "code_interpreter"}]
)

print(assistant)