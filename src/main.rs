use serde::{Deserialize, Serialize};
use alloy_primitives::I256;
use serde_json::{json, value::Value};

// In this example, expected response for either the eth_blockNumber, eth_gasPrice RPC methods will get a JSON result with the following keys => "id", "jsonrpc", "result", 
// Those keys should should match the fields in the struct below, which will then capture the value that is stored in the 'value' portion for each key within the Json data structure as a String in this Rust struct.
#[derive(Serialize, Deserialize, Debug)]
struct EthRPCJsonResponse {
    id: String,
    jsonrpc: String,
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?; //This loads the environment variables in your .env file. **DON'T PUBLISH A NODE API KEY - use that .env file!!**
    let client = reqwest::Client::new();
    //The "eth_gasPrice" JSON RPC method is hardcoded here. Substitute in 'eth_blockNumber' for a different value to appear in the printout.
    let request: Value = json!({"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":"1"});
    let response = client.post(dotenvy::var("NODE_API").unwrap())
        .json(&request) //.json function takes a reference to T; T must have the serde:ser:Serialized trait, which is a reference to &self,
        //in this case, the reference to &self is the variable 'response', of Rust type serde_json::Value
        .send() //While this function sends the &request to the Ethereum node, it returns a response type
        .await?; // We get a Future, which we draw out with 'await', and then we get the response type Response, which we draw out with the ? operator
    
    let status: reqwest::StatusCode = response.status(); //status method only takes a reference to 'response' variable, i.e the entire JSON response, as a whole 
    match status {
        reqwest::StatusCode::OK => {
            println!("Success!");
        },
        reqwest::StatusCode::NOT_FOUND => {
            println!("Got 404! Haven't found resource!");
        },
        _=> {
            panic!("Okay... this shouldn't happen...");
        }
    }

    //Allows 'response' type to be moved into 'json_response' object, of type EthRPCJsonResponse.
    let json_response = response.json::<EthRPCJsonResponse>().await?; 
    //Retrieve the value from the 'result' field of the EthRPCJsonResponse struct.
    let result_value = &json_response.result;
    // The Turbofish syntax with the parse function tells the compiler what datatype we want to turn our String from the EthRPCJsonResponse struct into
    // Since EthRPCJsonResponse.result is a hexadecimal value, the intended data type could be an I265. See variable 'C' under the usage section: https://docs.rs/alloy-primitives/0.3.0/alloy_primitives/struct.Signed.html
    let latest_value = &result_value.parse::<I256>().unwrap();
    
    println!("Your response status code is: {:?}", status);
    println!("The full JSON response was a:"); 
    println!("{:#?}", &json_response);
    println!("The latest integer value you want is: {:?}", &latest_value);

    Ok(())
}

