# $(date -u) returns current time in UTC, so grab that when running
# the command to populate into the next iteration
#
# returns a json blob with the book for one of BZX, BYX, EDGX, EDGA
# TODO: 
curl 'https://www.cboe.com/json/bzx/book/SPY' \
  -H 'authority: www.cboe.com' \
  -H 'accept: */*' \
  -H 'accept-language: en-US,en;q=0.9' \
  -H 'dnt: 1' \
  -H 'if-modified-since: Thu, 23 Mar 2023 19:37:24 GMT' \
  -H 'referer: https://www.cboe.com/us/equities/market_statistics/book/SPY/' \
  -H 'sec-ch-ua: "Microsoft Edge";v="111", "Not(A:Brand";v="8", "Chromium";v="111"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Windows"' \
  -H 'sec-fetch-dest: empty' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-site: same-origin' \
  -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.51' \
  -H 'x-requested-with: XMLHttpRequest' \
  --compressed
