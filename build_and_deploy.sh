#!/bin/bash

PORT=4000
ec2_ipv4_public_dns=$EC2_IPV4_PUBLIC_DNS

cross build --release --target x86_64-unknown-linux-gnu

ssh -i ./keys/personal-website.pem ubuntu@$ec2_ipv4_public_dns "lsof -t -iTCP:4000 -sTCP:LISTEN | xargs -r kill -9"

# copy the binary
scp -i ./keys/personal-website.pem ./target/x86_64-unknown-linux-gnu/release/website ubuntu@$ec2_ipv4_public_dns:~/website/

# copy any static files
scp -i ./keys/personal-website.pem -r ./static ubuntu@$ec2_ipv4_public_dns:~/website/

ssh -i ./keys/personal-website.pem ubuntu@$ec2_ipv4_public_dns "cd ~/website && chmod +x website && nohup ./website > website.log 2>&1 &"
