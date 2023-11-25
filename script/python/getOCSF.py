import requests
import json
from datetime import datetime, timedelta
import time

def get_and_save_json(url, filename):
    # Fetch JSON data from the URL
    response = requests.get(url)
    
    # Check if the request was successful (status code 200)
    if response.status_code == 200:
        # Parse JSON data
        json_data = response.json()
        
        # Save JSON data to the specified file
        with open(filename, 'a') as file:
            json.dump(json_data, file, indent=2)
            file.write('\n \n \n \n \n')  # Add a newline between JSON objects to keep them separate
        
        print(f"JSON data saved to {filename}")
    else:
        print(f"Failed to fetch JSON data. Status code: {response.status_code}")

# Replace the URL with your actual URL
url = "http://192.168.43.3:8080/sample/classes/account_change?profiles="

# Set the duration for the script to run (5 minutes)
duration = 5 * 60 

# Generate a timestamp for the filename
timestamp = datetime.now().strftime("%Y%m%d%H%M%S")
filename = f"ocsf_schema_sample_data{timestamp}.json"

start_time = datetime.now()

while (datetime.now() - start_time).seconds < duration:
    get_and_save_json(url, filename)
    time.sleep(1) 
print("Script completed.")



