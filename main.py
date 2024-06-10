from openai import OpenAI
from dotenv import load_dotenv
import os
import time
from assistant.instructions import (
    human_transcribed_response,
    problem_proposer_ai_prompt,
    initial_design_ai_prompt,
    stellar_interface_ai_prompt,
    security_ai_prompt,
    final_design_ai_prompt,
    extraction_ai_prompt,
    build_ai_prompt,
    tml_ai_prompt,
    documentation_ai_prompt,
    one_file_smart_contract_ai
    
)

load_dotenv()

api_key = os.getenv("API_KEY")
assistant_id = os.getenv("ASSISTANT_ID")
client = OpenAI(api_key=api_key)

######################### ASSISTANT RUN FUNCTIONS #########################
def create_thread():
    thread = client.beta.threads.create()
    return thread.id

def add_message(thread_id, role, content): 
    message = client.beta.threads.messages.create(
        thread_id=thread_id,
        role=role,
        content=content
    )
    return message

def create_run_and_poll(thread_id, assistant_id, instructions=""):
    run = client.beta.threads.runs.create_and_poll(
        thread_id=thread_id,
        assistant_id=assistant_id,
        instructions=instructions
    )
    print(run) 
    i = 0
    while True:
        run_status = client.beta.threads.runs.retrieve(
            thread_id=thread_id,
            run_id=run.id
        )
        print(f"run {i}")
        i += 1
        if run_status.status == 'completed':
            thread_messages = client.beta.threads.messages.list(thread_id, order="desc")
            message = thread_messages.data[0]  # Access the first message
            content_block = message.content[0]  # Access the first content block
            text = content_block.text  # Access the text object
            value = text.value
            return value
        elif run_status.status == 'failed':
            raise Exception(f"Run failed: {run_status['error']}")
        time.sleep(5)

def save_file(file_content, file_name):
    with open(file_name, 'w') as file:
        file.write(file_content)
    print(f"Saved {file_name}")

def module_iteration(thread_id, assistant_id, previous_message, ai_prompt):
    add_message(thread_id, "assistant", previous_message)
    response_text = create_run_and_poll(thread_id, assistant_id, ai_prompt)
    if response_text is None:
        # Handle the case where 'value' was not found
        return "Skipping iteration due to missing 'value' key."
    print(response_text)
    return response_text

def clean_file(ai_response):   
    completion = client.chat.completions.create(
    model="gpt-3.5-turbo-0125",
    messages=[
        {"role": "system", "content": "You are about to recieve the code for a rust or tml file. It may have external comments that will make the code not work (they are not correctly formatted). Output the code only without those mal-formed comments."},
        {"role": "user", "content": ai_response}
    ]
    )
    return completion.choices[0].message.content


######################### WORKFLOW PHASES #########################
def understand_problem(assistant_id, human_transcribed_response, problem_proposer_ai_prompt):
    understand_thread_id = create_thread()
    problem_proposer_response_text = module_iteration(understand_thread_id, assistant_id, human_transcribed_response, problem_proposer_ai_prompt)
    return problem_proposer_response_text

def initial_design_and_iteration(assistant_id, problem_proposer_response_text, initial_design_ai_prompt, stellar_interface_ai_prompt):
    initial_design_thread_id = create_thread()
    
    # Initial Design
    initial_design_response_text = module_iteration(initial_design_thread_id, assistant_id, problem_proposer_response_text, initial_design_ai_prompt)
    
    # Stellar Interfacing expert
    interface_expert_response_text = module_iteration(initial_design_thread_id, assistant_id, f"Initial design proposed by Expert designer: {initial_design_response_text}", stellar_interface_ai_prompt)

    # Security expert
    security_expert_response_text = module_iteration(initial_design_thread_id, assistant_id, f"Improvement suggestions made by the expert in interfacing with the Stellar ecosystem: {interface_expert_response_text}", security_ai_prompt)

    # Final designer
    final_design_response_text = module_iteration(initial_design_thread_id, assistant_id, f"Improvement suggestions made by the Stellar ecosystem security expert: {security_expert_response_text}", final_design_ai_prompt)
    return final_design_response_text

def contract_build(assistant_id, problem_proposer_response_text, initial_design_and_iteration_response_text, builder_ai_prompt, extraction_ai_prompt):
    # Extract file names
    extract_thread_id = create_thread()
    print(extract_thread_id)
    file_names_response_text = module_iteration(extract_thread_id, assistant_id, initial_design_and_iteration_response_text, extraction_ai_prompt)
    file_names_list = [file_name.strip() for file_name in file_names_response_text.split(',')]
    print(file_names_list)

    # Build rust files
    build_thread_id = create_thread()
    print(build_thread_id)
    add_message(build_thread_id, "assistant", problem_proposer_response_text)
    add_message(build_thread_id, "assistant", initial_design_and_iteration_response_text)
    os.makedirs("contract_files/src")
    for file_name in file_names_list:
        message = add_message(build_thread_id, "user", f"Your task is to create the code for {file_name}")
        print(message) 
        file_build_response_text = create_run_and_poll(build_thread_id, assistant_id, builder_ai_prompt)
        cleaned_text = clean_file(file_build_response_text)
        save_file(cleaned_text, f"contract_files/src/{file_name}")
        add_message(build_thread_id, "assistant", f"{file_name} generated code = {cleaned_text}")
        print(cleaned_text)
    
    return build_thread_id 
    
def finalize_build(assistant_id, build_thread_id, one_file_smart_contract_ai, tml_ai_prompt, documentation_ai_prompt):
    # ONE CONTRACT
    whole_contract_file_response_text = create_run_and_poll(build_thread_id, assistant_id, instructions=one_file_smart_contract_ai)
    save_file(whole_contract_file_response_text, f"contract_files/src/whole_contract.rs")
    

    # GENERATE TOML
    toml_file_response_text = create_run_and_poll(build_thread_id, assistant_id, instructions=tml_ai_prompt)
    save_file(toml_file_response_text, f"contract_files/cargo.toml")
    add_message(build_thread_id, "assistant", f"code generated by the tml expert: {toml_file_response_text}")
    
    # GENERATE DOCUMENTATION
    docs_file_response_text = create_run_and_poll(build_thread_id, assistant_id, instructions=documentation_ai_prompt)
    save_file(docs_file_response_text, f"contract_files/README.md")
    add_message(build_thread_id, "assistant", f"code generated by the documentation expert: {docs_file_response_text}")
    return 

######################### MAIN PROGRAM #########################
def main():
    start_time = time.time()
    problem_proposer_response_text = understand_problem(assistant_id, human_transcribed_response, problem_proposer_ai_prompt)
    initial_design_and_iteration_response_text = initial_design_and_iteration(assistant_id, problem_proposer_response_text, initial_design_ai_prompt, stellar_interface_ai_prompt)
    build_thread_id = contract_build(assistant_id, problem_proposer_response_text, initial_design_and_iteration_response_text, build_ai_prompt, extraction_ai_prompt)
    finalize_and_document = finalize_build(assistant_id, build_thread_id, one_file_smart_contract_ai, tml_ai_prompt, documentation_ai_prompt)
    end_time = time.time()
    print(f"PROCESS FINISHED - TIME TAKEN: {end_time - start_time} seconds")
    
    
    
if __name__ == "__main__":
    main()


