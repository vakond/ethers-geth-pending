#!/usr/bin/env bash
# This script is intended to run in a container automatically.
# It would be useless to try launch it out of that context.

# Exit on failure
set -ex

# Turn on job control
set -m

HOST=0.0.0.0 # listen all addresses to work in a container properly
PORT=8545

geth init /root/genesis.json

rm -rf /root/.ethereum/keystore
cp -r /root/keystore /root/.ethereum/

geth version &> geth.log

exec geth \
    --nousb \
    --nodiscover \
    --unlock "0xff93B45308FD417dF303D6515aB04D9e89a750Ca","0x8e0a907331554AF72563Bd8D43051C2E64Be5d35","0x24962717f8fA5BA3b931bACaF9ac03924EB475a0","0x148FfB2074A9e59eD58142822b3eB3fcBffb0cd7","0x4CEEf6139f00F9F4535Ad19640Ff7A0137708485" \
    --password /root/password.txt \
    --rpc \
    --rpcport $PORT \
    --rpccorsdomain="*" \
    --rpcaddr $HOST \
    --ws \
    --wsport $PORT \
    --wsorigins="*" \
    --wsaddr $HOST \
    --networkid 5 \
    --targetgaslimit 8000000 \
    --allow-insecure-unlock \
    --mine &>> geth.log &

echo
echo "Geth listening on $HOST:$PORT..."
echo

fg %1
