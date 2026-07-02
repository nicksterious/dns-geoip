pip install dnspython

import dns.resolver

# Querying an A record (IPv4)
answers = dns.resolver.resolve("188.12.23.34.geo.ipns.cc", "TXT")
for rdata in answers:
    print(f"IP Address: {rdata.address}")
