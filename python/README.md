# Python Bluefin Pro SDK

## Example

The ./example directory contains a full example of using the SDK, end-to-end.

## Contributing

### Setup

Change into the `sdk` directory, and run the following commands to set up your
virtual environment and install the requirements, then the testing dev
requirements:

```bash
# Or use poetry from the directory `sdk`.  Run the following commands  to
# install poetry if needed.
pipx install poetry

# Set it to use python3.12 (making sure you have python 3.12 installed).
# If you don't have python 3.12 installed, you can use pyenv to install it:
# pyenv install 3.12

# From the `python` directory, run the following commands:
python3.12 -m venv .venv 
source .venv/bin/activate
poetry install
```

If you are using IntelliJ, make sure to configure it to point to the virtual env
executable under `.venv/bin/python` created from previous commands.  The example
directory contains a main method that fully runs the SDK functions.

Now you can run the example in the ./example dir by running `python main.py`.


Generating the OpenAPI Client
----------------------------

The OpenAPI client code in the ``sdk/src`` directory is automatically generated using the OpenAPI Generator.
To regenerate the client code, run (from the ``sdk`` directory):

```bash
openapi-generator generate -i ../../resources/bluefin-api.yaml -c config.yaml -g python -o src
```

This will:

1. Use the OpenAPI spec from ``../../resources/bluefin-api.yaml``
2. Apply configuration from ``config.yaml`` 
3. Generate a Python client
4. Output the generated code to the ``src`` directory