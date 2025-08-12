#!/usr/bin/env bash
# generate_certs.sh

# Create a configuration file for the certificate
cat > server.conf <<EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
C = US
ST = State
L = City
O = Bitcoin Proxy
CN = localhost

[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = *.localhost
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

# Generate private key
openssl genrsa -out key.pem 2048

# Generate certificate signing request
openssl req -new -key key.pem -out server.csr -config server.conf

# Generate the certificate
openssl x509 -req -in server.csr -signkey key.pem -out cert.pem \
    -days 365 -extensions v3_req -extfile server.conf

# Clean up
rm server.csr server.conf

echo "Generated key.pem and cert.pem"
echo "Verifying certificate..."
openssl x509 -in cert.pem -text -noout | grep -A 2 "Basic Constraints"
