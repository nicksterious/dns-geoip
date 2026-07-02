
require 'resolv'

txt = Resolv::DNS.open do |dns|
  records = dns.getresources("188.12.23.34.geoip.adstart.cc", Resolv::DNS::Resource::IN::TXT)
  records.empty? ? nil : records.map(&:data).join(" ")
end
