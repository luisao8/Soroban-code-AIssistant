import os

# Define the base directory
base_dir = '/Users/luisalarcon/Documents/scraped_docs'  # Change this to the path of your folder

# List to store file paths
file_paths = []


# # Iterate over each folder in the base directory
# for folder_name in os.listdir(base_dir):
#     folder_path = os.path.join(base_dir, folder_name)
    
#     # Check if it's a directory
#     if os.path.isdir(folder_path):
#         src_path = os.path.join(folder_path, 'src')
        
#         # Check if the 'src' directory exists
#         if os.path.exists(src_path):
#             lib_file = os.path.join(src_path, f'{folder_name}_lib.txt')
#             test_file = os.path.join(src_path, f'{folder_name}_test.txt')
            
#             # Rename lib.rs to {parent_folder_name}_lib.txt
#             if os.path.exists(lib_file):
#                 file_paths.append(lib_file)
#                 print(f"Renamed {lib_file} to {lib_file}")
            
#             # Rename test.rs to {parent_folder_name}_test.txt
#             if os.path.exists(test_file):
                
#                 file_paths.append(test_file)
#                 print(f"Renamed {test_file} to {test_file}")

# # Print the list of renamed file paths
# print("File paths:", file_paths)

for folder_name in os.listdir(base_dir):
    file_paths.append(os.path.join(base_dir, folder_name))

print(file_paths)

