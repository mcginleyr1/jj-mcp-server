#!/usr/bin/env python3
"""
Comprehensive test of the jj MCP server
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

def test_tool(tool_name, arguments):
    """Test a specific tool with arguments"""
    print(f"Testing {tool_name} tool...")
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        }
    }
    
    response = send_mcp_request(request)
    if response and 'result' in response:
        content = response['result']['content'][0]['text']
        is_error = response['result'].get('isError', False)
        print(f"✓ {tool_name}: {'ERROR' if is_error else 'SUCCESS'}")
        print(f"  Output: {content[:100]}...")
        return not is_error
    else:
        print(f"✗ {tool_name}: FAILED")
        if response:
            print(f"  Response: {response}")
        return False

if __name__ == "__main__":
    print("Comprehensive jj MCP Server Test")
    print("================================")
    
    tests = [
        ("status", {}),
        ("log", {"limit": 3}),
        ("diff", {"summary": True}),
        ("new", {"parents": "@"}),
        ("commit", {"message": "Test commit from MCP server"}),
    ]
    
    passed = 0
    total = len(tests)
    
    for tool_name, args in tests:
        if test_tool(tool_name, args):
            passed += 1
        print()
    
    print(f"Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! The jj MCP server is working correctly.")
    else:
        print("⚠️  Some tests failed. Check the output above for details.")