<?php
  $results = dns_get_record('188.12.23.34.geoip.adstart.cc', DNS_TXT);
  print( $results[0]["txt"] );
?>
