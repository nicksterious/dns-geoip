# Warning
This is a public beta NOT suitable for production.

# What is this?
A free and super fast GeoIP service.

```
dig -t txt 188.12.23.34.geoip.adstart.cc
```
```
=> 188.12.23.34.geoip.adstart.cc. 86400 IN	TXT	"|44.1307|10.0218|Fosdinovo|54035||Tuscany|Tuscany|IT|Italy|Europe"
```

## What is all that?

| ASN (missing) | Long    | Lat     | City      | Post code | State (missing) | County  | Region  | Country ISO2 code | Country name | Continent |
|---------------:|:--------|:--------|:----------|:----------|:----------------:|:--------|:--------|:-----------------:|:------------:|:---------:|
|                | 44.1307 | 10.0218 | Fosdinovo | 54035     |                  | Tuscany | Tuscany | IT                | Italy        | Europe    |


## Notes

ASN is WIP I'll be adding this once I get a database.

"State" isn't available in the EU.

Right now some IP addresses in low-density countries/areas will show up as Unknown as pretty much nobody fully covers them on a city / post code level.

If a a query is only partially resolved some missing data may show up as "Unknown", eg:
```
dig -t txt 145.12.23.34.geoip.adstart.cc
```
```
=> "|52.3824|4.8995|Unknown|0||Unknown|Unknown|NL|The Netherlands|Europe"
```

Some Google IP addresses will show up as GoogleBot with unspecified location.
```
dig -t txt 66.249.100.100.geoip.adstart.cc
```
```
=> "|37.43|-78.65|GoogleBot||||VA|US|United States|"
```

WIP: spam bots / vulnerability scanners / scrapers / exploit scanners will show up as BadBot
```
dig -t txt x.x.x.x.geoip.adstart.cc
```
```
=> "|0|0|BadBot|||||XX|International|"
```

WIP: AI bots that send thousands of requests to your servers eating up your resources/CPU/RAM/bandwidth so that some rich corporations can make billions will show up as AIBot
```
dig -t txt x.x.x.x.geoip.adstart.cc
```
```
=> "|0|0|AIBot|||||XX|International|"
```


# Why?
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

Most languages have native support for DNS resolution, in most (civilized) cases without additional dependencies, see code samples.

# Roadmap
If this gains positive feedback/traction I will work on scaling for capacity and contract a commercial, higher accuracy GeoIP database.

# Can I help
Not really, there's not much code to write and the architecture is quite simple, but here are a few ways you can help

## Code samples
You're welcome to send a PR with code samples in your preferred language.

## Suggestions
Please start a discussion if you have feedback or ideas.
