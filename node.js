const dns = require("dns");

dns.resolveTxt('145.12.23.34.geo.ipns.cc', (err, addresses) => console.log('TXT records: %j', addresses));

geoip_data = addresses.split("|");
