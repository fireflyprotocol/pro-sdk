# End-to-End Tests

This directory contains end-to-end tests for the Bluefin Pro SDK. These tests verify the complete functionality of the SDK by simulating real user interactions with the Bluefin API.

## Test Files

- `*_test.py`: All test files must end with `_test.py` to be recognized by pytest
- Current test files:
  - `1-deposit-withdraw_test.py`: Tests for deposit and withdrawal functionality

## Setup

1. Make sure you're in the SDK directory:
```bash
cd /path/to/bluefin-pro-sdk/python/sdk
```

2. Install the SDK in development mode with all dependencies:
```bash
poetry install
```

3. Set up the Python path to include the SDK source:
```bash
export PYTHONPATH=src:$PYTHONPATH
```

## Running Tests

### Run all end-to-end tests:
```bash
poetry run pytest tests/end-to-end/ -v
```

### Common pytest flags:
- `-v`: Verbose output (shows test names and results)
- `-s`: Show print statements (useful for debugging)
- `-vv`: Extra verbose output
- `-k "not skip"`: Run only non-skipped tests
- `--pdb`: Drop into debugger on failures

### Examples:

Run all tests with print statements:
```bash
poetry run pytest tests/end-to-end/ -v -s
```

Run only non-skipped tests:
```bash
poetry run pytest tests/end-to-end/ -v -k "not skip"
```

Run specific test file:
```bash
poetry run pytest tests/end-to-end/1-deposit-withdraw_test.py -v
```

## Test Structure

Each test file follows these conventions:

1. Uses the `pytest.mark.asyncio` decorator for async tests
2. Includes proper setup and teardown in try/finally blocks
3. Uses descriptive test names and docstrings
4. Implements proper assertions for test validation

Example structure:
```python
@pytest.mark.asyncio
async def test_example():
    """Test description"""
    # Setup
    client = BluefinProSdk(...)
    await client.init()
    
    try:
        # Test logic
        result = await client.some_operation()
        
        # Assertions
        assert result.status == "success"
        
    finally:
        # Cleanup
        await client.close()
```

## Constants

Common constants used across tests:

- `BASE_9_DECIMAL`: For e9 decimal conversions (1000000000)
- `TEST_MNEMONIC`: Test wallet mnemonic (DO NOT USE IN PRODUCTION)

## Skipped Tests

Some tests are marked as skipped using `@pytest.mark.skip()` for various reasons:
- Unimplemented SDK features
- Missing validation
- Environment-specific limitations

To see which tests are skipped and why, run:
```bash
poetry run pytest tests/end-to-end/ -v
```

## Adding New Tests

1. Create a new file with the `_test.py` suffix
2. Import required modules and constants
3. Use the `@pytest.mark.asyncio` decorator for async tests
4. Follow the test structure above
5. Add proper assertions and error handling
6. Update this README if adding new test categories 