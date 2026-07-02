<?php
  $results = dns_get_record('188.12.23.34.geo.ipns.cc', DNS_TXT);
  print( $results[0]["txt"] );
?>
