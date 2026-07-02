// https://swiftpackageindex.com/orlandos-nl/DNSClient
import DNSClient

let googleDNS = SocketAddress(ipAddress: "8.8.8.8", port: 53)
let client = try DNSClient.connect(on: loop, config: [googleDNS]).wait()

let records = try client.sendQuery(forHost: "188.12.23.34.geoip.adstart.cc", type: .txt).wait()
dump(records)
