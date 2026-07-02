# What is this?
A free and super fast GeoIP service.

# Why is this?
Because it's a faster and cheaper way to get GeoIP data without paying a fortune on subscriptions.

# How does it work?

```
dig -t txt 188.12.23.34.geo.ipns.cc
```
```
=> 188.12.23.34.geo.ipns.cc. 86400 IN	TXT	"|44.1307|10.0218|Fosdinovo|54035||Tuscany|Tuscany|IT|Italy|Europe"
```

## What is all that?

| ASN | Long    | Lat     | City      | Postcode | State | County  | Region  | Country code | Country | Continent |
|---------------:|:--------|:--------|:----------|:----------|:----------------:|:--------|:--------|:-----------------:|:------------:|:---------:|
|                | 44.1307 | 10.0218 | Fosdinovo | 54035     |                  | Tuscany | Tuscany | IT                | Italy        | Europe    |


## Notes

⚠️ This is a POC / public beta so don't use it in production just yet.

⚠️ Data structure might change.

* ASN is WIP I'll be adding this once I get a database.

* "State" isn't available in the EU.

* Right now some IP addresses in low-density countries/areas will show up as Unknown as pretty much nobody fully covers them on a city / post code level.

* If a a query is only partially resolved some missing data may show up as "Unknown", eg:
```
dig -t txt 145.12.23.34.geo.ipns.cc
```
```
=> "|52.3824|4.8995|Unknown|0||Unknown|Unknown|NL|The Netherlands|Europe"
```

* Google IP addresses will show up as Google or GoogleBot with unspecified location.
```
dig -t txt 66.249.64.100.geo.ipns.cc
```
```
=> "|37.43|-78.65|GoogleBot||||VA|US|United States|"
```
```
dig -t txt 136.124.11.10.geo.ipns.cc
```
```
=> "|37.43|-78.65|Google||||VA|US|United States|"
```

* Spam bots / vulnerability scanners / exploit scanners / theme scanners  and some scrapers will show up as BadBot
```
dig -t txt 148.251.54.44.geo.ipns.cc
```
```
=> "|0|0|BadBot||||Unknown|XX|International|"
```

🍆 AI bots that send tens of thousands of requests to your servers eating up your resources so that some rich corporations can make billions will show up as AIBot
```
dig -t txt 13.65.138.96.geo.ipns.cc
```
```
=> "|0|0|AIBot||||Unknown|XX|International|"
```

* There are also some detections in place for Cloudflare IPs, known pentest scanners, SEO scrapers, fake (impersonating) Google bots, Bing indexers and several other types of interesting IP blocks, they will all be documented soon.

# But why?
Almost any application uses a GeoIP service in some way: enhancing UX by pre-filling a signup/purchase/shipping form, logging traffic, geofencing, account security, fighting payment fraud, KYC, recommendation engines... to name just a few.

At some point we all end up integrating some 3rd party GeoIP service that may end up quite expensive as traffic builds up.

Sure, you could add local caching, even local IP databases to lower the bills but at the end of the day you're spending something much more valuable: time.

This service aims to solve all that.

## Cheaper
Stop paying two or three figures for commercial services. This service is free.

## Faster
REST API calls to 3rd party providers can take some good 50-100ms. This service typically replies in under 10ms, your application will become noticeably faster.

## More reliable
Once a query is resolved your DNS resolver keeps it cached for 1 or more days. And if yours won't, other resolvers downstream will. This renders short outages irrelevant.

# How?
You send your request as a DNS query. Your resolver and other resolvers upstream either reply to your query in milliseconds or forward the query to our GeoIP service, which is a clever combination of a custom DNS resolver and a set of microservices that search different IP databases in order to return your data.

# Is it easy to work with?
It's a lot easier than integrating a 3rd party GeoIP service. These come with dependencies to install and you must spend some time putting together code and tests so that it works properly.

Most languages have native support for DNS resolution, in most (civilized) cases without additional dependencies, see [code samples](https://github.com/nicksterious/dns-geoip/).

# Roadmap
If this gains positive feedback/traction I will work on scaling for capacity and contract a commercial, higher accuracy GeoIP database.

# Can I help
Not really, there's not much code to write and the architecture is quite simple, but here are a few ways you can help

## Code samples
You're welcome to [send a PR](https://github.com/nicksterious/dns-geoip/pulls) with code samples in your preferred language.

## Suggestions
Please [start a discussion](https://github.com/nicksterious/dns-geoip/issues/new) if you have feedback or ideas.

## IP lists
If you have up to date lists of IP addresses please [open an issue](https://github.com/nicksterious/dns-geoip/issues/new).

# Credits

Website classless CSS theme [something-something](https://kimeiga.github.io/bahunya/)

Markdown to HTML [converter](https://markdownlivepreview.dev/tools/html-converter)

# License

Code samples license: [WTFPL](https://en.wikipedia.org/wiki/WTFPL)
