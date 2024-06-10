from openai import OpenAI
from dotenv import load_dotenv
import os
import mimetypes
# Load environment variables from .env
load_dotenv()

# Now you can access your variables like this

api_key = os.getenv("API_KEY")
client = OpenAI(api_key=api_key)
assistant_id = os.getenv("ASSISTANT_ID")
vector_store_id = os.getenv("VECTOR_STORE_ID")


id = assistant_id 

def get_file_paths(directory):
    file_paths = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".txt"):
                file_paths.append(os.path.join(root, file))
    return file_paths

# Directory containing the scraped .txt files
files_directory = "--------------------"

# Get all file paths in the directory
file_paths = get_file_paths(files_directory)
print(len(file_paths))
file_streams = [open(path, "rb") for path in file_paths]

print(file_streams)

file_batch = client.beta.vector_stores.file_batches.upload_and_poll(
  vector_store_id=vector_store_id, files=file_streams
)
 
# # You can print the status and the file counts of the batch to see the result of this operation.
# print(file_batch.status)
# print(file_batch.file_counts)