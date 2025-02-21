import requests
import json
import time

LOCKED_OBJECT_ERROR_CODE = (
    "Failed to sign transaction by a quorum of validators because of locked objects"
)


def rpc_unsafe_moveCall(
    url: str,
    params: list,
    function_name: str,
    function_library: str,
    userAddress: str,
    packageId: str,
    gasBudget: int = 100000000,
    typeArguments: list = [],
):
    """
    Does the RPC call to SUI chain
    Input:
      url: url of the node
      params: a list of arguments to be passed to function
      function_name: name of the function to call on sui
      function_library: name of the library/module in which the function exists
      userAddress: Address of the signer
      packageId: package id in which the module exists
      gasBudget(optional): gasBudget defaults to 100000000
      typeArguments (optional): type arguments if any in list format [your_arg_1]

    Output:
      Returns the request form serialized in bytes ready to be signed.

    """
    base_dict = {}
    base_dict["jsonrpc"] = "2.0"
    base_dict["method"] = "unsafe_moveCall"
    base_dict["id"] = 1
    base_dict["params"] = []
    base_dict["params"].extend(
        [userAddress, packageId, function_library, function_name]
    )

    # Optional type arguments for wormhole related calls
    base_dict["params"].append(typeArguments)
    base_dict["params"].append(params)

    base_dict["params"].append(None)
    base_dict["params"].append(str(gasBudget))

    payload = json.dumps(base_dict)

    headers = {"Content-Type": "application/json"}
    response = requests.request("POST", url, headers=headers, data=payload)
    responseJson = json.loads(response.text)
    if "result" not in responseJson or "txBytes" not in responseJson["result"] :
            raise (Exception(f"Failed to create transaction bytes due to: {responseJson}"))
    return responseJson["result"]["txBytes"]


def rpc_sui_executeTransactionBlock(url, txBytes, signature, maxRetries=5):
    """
    Execute the SUI call on sui chain
    Input:
      url: url of the node
      txBytes: the call in serialized form
      signature: txBytes signed by signer

    Output:
      result of transaction


    """
    base_dict = {}
    base_dict["jsonrpc"] = "2.0"
    base_dict["id"] = 5
    base_dict["method"] = "sui_executeTransactionBlock"
    base_dict["params"] = []
    base_dict["params"].append(txBytes)
    base_dict["params"].append([signature])

    outputTypeDict = {
        "showInput": True,
        "showEffects": True,
        "showEvents": True,
        "showObjectChanges": True,
    }
    base_dict["params"].append(outputTypeDict)
    base_dict["params"].append("WaitForLocalExecution")
    payload = json.dumps(base_dict)

    headers = {"Content-Type": "application/json"}

    for i in range(0, maxRetries):
        response = requests.request("POST", url, headers=headers, data=payload)
        result = json.loads(response.text)
        if "error" in result:
            if result["error"]["message"].find(LOCKED_OBJECT_ERROR_CODE) == -1:
                return result
        else:
            return result

        time.sleep(1)
    return result

def rpc_sui_getDynamicFieldObject(url:str, parentObjectId: str, fieldName: str,fieldSuiObjectType:str, maxRetries=5):
    """
    Fetches the on-chain dynamic field object corresponding to specified input params
    Input:
      url: url of the sui chain node
      parentObjectId: id of the parent object for which dynamic field needs to be queried
      fieldName: name of the dynamic field
      fieldSuiObjectType: sui object type for the dynamic field name (eg. for string use , `0x1::string::String`)

    Output:
      sui result object for the dynamic field
    """
    base_dict = {}
    base_dict["jsonrpc"] = "2.0"
    base_dict["id"] = 5
    base_dict["method"] = "suix_getDynamicFieldObject"
    base_dict["params"] = []
    base_dict["params"].append(parentObjectId)
    base_dict["params"].append({
         "type": fieldSuiObjectType,
         "value": fieldName
    })
    payload = json.dumps(base_dict)

    headers = {"Content-Type": "application/json"}

    for i in range(0, maxRetries):
        response = requests.request("POST", url, headers=headers, data=payload)
        result = json.loads(response.text)
        if "error" in result:
            if result["error"]["message"].find(LOCKED_OBJECT_ERROR_CODE) == -1:
                return result
        else:
            return result

        time.sleep(1)
    return result

def rpc_call_sui_function(url, params, method="suix_getCoins"):
    """
    for calling sui functions:
    Input:
      url: url of node
      params: arguments of function
      method(optional): the name of method in sui we want to call. defaults to suix_getCoins
    """
    base_dict = {}
    base_dict["jsonrpc"] = "2.0"
    base_dict["id"] = 1
    base_dict["method"] = method
    base_dict["params"] = params
    payload = json.dumps(base_dict)

    headers = {"Content-Type": "application/json"}
    response = requests.request("POST", url, headers=headers, data=payload)
    result = json.loads(response.text)
    return result["result"]["data"]

def get_coins(user_address: str = None, coin_type: str = "0x::sui::SUI", url: str = None):
        """
        Returns the list of the coins of type tokenType owned by user
        """
        try:
            callArgs = []
            callArgs.append(user_address)
            callArgs.append(coin_type)
            result = rpc_call_sui_function(
                url, callArgs, method="suix_getCoins")
            return result
        except Exception as e:
            print(e)
            raise (Exception("Failed to get coins, Exception: {}".format(e)))
        
async def get_coin_balance(user_address: str = None, coin_type: str = "0x::sui::SUI", url: str = None) -> str:
        """
        Returns user's token balance.
        """
        try:
            callArgs = []
            callArgs.append(user_address)
            callArgs.append(coin_type)
            result = rpc_call_sui_function(
                url, callArgs, method="suix_getBalance"
            )["totalBalance"]
            return result
        except Exception as e:
            raise (Exception("Failed to get coin balance, Exception: {}".format(e)))


def get_coin_having_balance(user_address: str = None, coin_type: str = "0x::sui::SUI", balance: str = None , url: str = None, exact_match: bool = False) -> str:
        
        coin_list = get_coins(user_address, coin_type, url)
        
        for coin in coin_list:
            print(coin["balance"], balance, int(coin["balance"]) >= balance, exact_match)
            if exact_match:
                 if int(coin["balance"]) == int(balance):
                      return coin["coinObjectId"]
            elif int(coin["balance"]) >= balance:
                return coin["coinObjectId"]
        raise Exception(
            "Not enough balance available in single coin, please merge your coins"
        )