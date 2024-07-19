import json
import csv
import os
import re

result = []
keys = ['request_uri','_zl_timestamp','method','user_agent','organization_id','dre-function-name','x-zoho-fromservice','content-type','type','JSONString','stream_content']

def get_issue_type(json_str):
    type = ''
    pattern = r'(?<!")\b\d{4}-\d{2}-\d{2}\b(?!")' #date regex
    pattern2 = r"':|:'" #single quotes
    pattern3 = r",\]|\,}" # comma before bracket
    match = re.search(pattern, json_str)
    if match is not None:
        type = 'date'
        print('date')
    match = re.search(pattern2, json_str)
    if match is not None:
        type = 'single quotes'
        print('single')
    match = re.search(pattern3, json_str)
    if match is not None:
        type = 'comma before bracket'
        print('comma before bracket')
    #return match is not None
    return type

def getTotalTime(fileName: str):
    print(fileName)
    with open(fileName,"r") as file:
        data = json.load(file)

        for docData in data['docs']:
            row = {}
            row['request_uri'] = str(docData['request_uri'])
            row['_zl_timestamp'] = str(docData['_zl_timestamp'])
            row['method'] = str(docData['method'])
            for param in docData['request_headers']:
                if param['name'] in ['dre-function-name','x-zoho-fromservice','content-type'] or str(param['name']).__contains__("organizationid"):
                    row[str(param['name'])] = str(param['value'])
                    if param['name'] not in keys:
                        keys.append(param['name'])
            if 'user_agent' in docData:
                row['user_agent'] = str(docData['user_agent'])
            if 'params' in docData:
                for param in docData['params']:
                    if param['param_name'] in ['organization_id','JSONString']:
                        row[str(param['param_name'])] = str(param['param_value'])
                    if 'JSONString' == param['param_name']:
                        row['type'] = str(get_issue_type(str(param['param_value'])))
            if 'stream_content' in docData:
                row['stream_content'] = str(docData['stream_content'])
                row['type'] = str(get_issue_type(docData['stream_content']))
            result.append(row)
        print("Completed")

directory_path = "/Users/esakki-20378/Downloads/Logs StaTS/sheets"
for filename in os.listdir(directory_path):
    if filename != 'final_result.csv' and not filename.startswith("."):
        getTotalTime(str(os.path.join(directory_path, filename)))

with open(directory_path+'/final_result.csv', 'w', newline='') as output_file:
    dict_writer = csv.DictWriter(output_file, keys)
    dict_writer.writeheader()
    dict_writer.writerows(result)


