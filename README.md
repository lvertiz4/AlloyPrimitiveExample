# Alloy Primitive Example
A simple example of using a data primitive from the Alloy library to make your Ethereum dapp more user-friendly


To run this example, copy this repo to your local machine. Add a .env to the repository and add an API key to a Node provider, like Infura or Alchemy. 

Notice on Line 20 that the referenced Ethereum node API is NODE_API. Make that the environment variable in your .env file.


After that, go to the terminal and run "CARGO RUN". Afterwards, change the "method' value from 'eth_gasPrice' to 'eth_blockNumber' to see a different value returns onto your terminal.
