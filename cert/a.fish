#!/bin/fish
# { cd ~/.certs/; openssl ca -config openssl-ca.cnf -policy signing_policy -extensions signing_req >a }
# echo sadfsdf | { cd ..; tee }
# ssh cachyos "openssl req -new -key /home/mq/docker/traefik/certs/rsa_4048.key -subj \"/CN=*\" -addext \"subjectAltName = DNS:*\"" |openssl ca -config openssl-ca.cnf -policy signing_policy -extensions signing_req 
yes "" | openssl req -config openssl-server.cnf -key rsa_4096.key -sha256 -nodes -outform PEM 2>/dev/null >~/.certs/req
