#!/usr/bin/env python3
"""
Test script for the jj MCP server
"""
import subprocess
import json
import sys

def send_mcp_request(request):
    """Send a JSON-RPC request to the MCP server"""
    try:
        result = subprocess.run(
            ['./target/release/jj-mcp-server'],
            input=json.dumps(request),
            text=True,
            capture_output=True,
            timeout=10
        )
        
        # The server outputs startup message to stderr
        print(f"stderr: {result.stderr}", file=sys.stderr)
        
        if result.stdout:
            try:
                return json.loads(result.stdout)
            except json.JSONDecodeError:
                print(f"Invalid JSON response: {result.stdout}")
                return None
        else:
            print("No stdout response")
            return None
            
    except subprocess.TimeoutExpired:
        print("Request timed out")
        return None
    except Exception as e:
        print(f"Error: {e}")
        return None

def test_initialize():
    """Test the initialize method"""
    print("Testing initialize...")
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {}
        }
    }
    
    response = send_mcp_request(request)
    if response:
        print(f"Initialize response: {json.dumps(response, indent=2)}")
        return True
    return False

def test_list_tools():
    """Test listing available tools"""
    print("Testing tools/list...")
    request = {
        "jsonrpc": "2.0", 
        "id": 2,
        "method": "tools/list"
    }
    
    response = send_mcp_request(request)
    if response:
        print(f"Tools list response: {json.dumps(response, indent=2)}")
        return True
    return False

def test_status_tool():
    """Test the status tool"""
    print("Testing status tool...")
    request = {
        "jsonrpc": "2.0",
        "id": 3, 
        "method": "tools/call",
        "params": {
            "name": "status",
            "arguments": {}
        }
    }
    
    response = send_mcp_request(request)
    if response:
        print(f"Status tool response: {json.dumps(response, indent=2)}")
        return True
    return False

if __name__ == "__main__":
    print("Testing jj MCP Server...")
    
    if test_initialize():
        print("✓ Initialize test passed")
    else:
        print("✗ Initialize test failed")
    
    if test_list_tools():
        print("✓ List tools test passed")
    else:
        print("✗ List tools test failed")
        
    if test_status_tool():
        print("✓ Status tool test passed") 
    else:
        print("✗ Status tool test failed")