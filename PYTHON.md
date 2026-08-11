# Python Usage

The Python script remains available for compatibility and reference.

The SDN list as XML file (~80 MB in May 2023) can be downloaded, for example,
via `wget`:

``` console
$ wget https://www.treasury.gov/ofac/downloads/sanctions/1.0/sdn_advanced.xml
```

By default, the python script expects the
[`sdn_advanced.xml`](https://www.treasury.gov/ofac/downloads/sanctions/1.0/sdn_advanced.xml)
XML file to be located in the current working directory. This can be changed with the
`-sdn /path/to/sdn_advanced.xml` argument.

To download the current XML file without using `wget` or `unzip`, run:

``` console
$ python3 generate-address-list.py fetch
```

By default this writes `sdn_advanced.xml` in the current directory. Use
`-o /path/to/sdn_advanced.xml` to choose another location.

---

By default, the `XBT` (Bitcoin) addresses are extracted into a `TXT` file:

``` console
$ python3 generate-address-list.py
$ cat sanctioned_addresses_XBT.txt | head -n2
12QtD5BFwRsdNsAZY76UVE1xyCGNTojH9h
1Kuf2Rd8mDyAViwBozGTNYnvWL8uYFrkVo
```

Other assets can be selected by supplying the tickers:

``` console
$ python3 generate-address-list.py ETH ETC DASH LTC
$ ls sanctioned_addresses_* -1
sanctioned_addresses_DASH.txt
sanctioned_addresses_ETC.txt
sanctioned_addresses_ETH.txt
sanctioned_addresses_LTC.txt
```

---

By default, a `TXT` file listing an address per line is produced. For example,
a JSON file listing the addresses can be produced with the `-f JSON` flag.

``` console
$ python3 generate-address-list.py XMR -f JSON
$ ls sanctioned_addresses_* -1
sanctioned_addresses_XMR.json
```

---

For detailed help and usage instructions please see `python3 generate-address-list.py --help`
