import os
from dotenv import load_dotenv
from openai import OpenAI
from instructions import designer_ai_prompt


load_dotenv()

api_key = os.getenv("API_KEY")
assistant_id = os.getenv("ASSISTANT_ID")
client = OpenAI(api_key=api_key)
 

name = "Stellar Interface Expert"
id = assistant_id 

assistant = client.beta.assistants.create(
  name=name,
  instructions=designer_ai_prompt,
  model="gpt-4o",
  tools=[{"type": "file_search"},{"type": "code_interpreter"}]
)

print(assistant)