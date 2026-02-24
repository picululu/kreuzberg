# 100G Networking Technology Overview

**Christopher Lameter** <cl@linux.com>
**Fernando Garcia** <fgarcia@dasgunt.com>

Toronto, August 23, 2016

---

## Why 100G now?

- Capacity and speed requirements on data links keep increasing.
- Fiber link reuse in the Connectivity providers (Allows Telcos to make better use of WAN links)
- Servers have begun to be capable of sustaining 100G to memory (Intel Skylake, IBM Power8+)
- Machine Learning Algorithms require more bandwidth
- Exascale Vision for 2020 of the US DoE to move the industry ahead.

---

## 100G Networking Technologies

- **10 x 10G Link old standard CFP C??:** Expensive. Lots of cabling. Has been in use for awhile for specialized uses.

- **New 4 x 28G link standards "QSFP28":** Brings down price to ranges of SFP and QSFP. Compact and designed to replace 10G and 40G networking.

- **Infiniband (EDR)**
  - Standard pushed by Mellanox.
  - Transitioning to lower Infiniband speeds through switches.
  - Most mature technology to date. Switches and NICs are available.

- **Ethernet**
  - Early deployment in 2015.
  - But most widely used chipset for switches recalled to be respun.
  - NICs are under development. Mature one is the Mellanox EDR adapter that can run in 100G Ethernet mode.
  - Maybe ready mid 2016.

- **Omnipath (Intel)**
  - Redesigned serialization. No legacy issues with Infiniband. More nodes. Designed for Exascale vision. Immature. Vendor claims production readiness but what is available has the character of an alpha release with limited functionality. Estimate that this is going to be more mature at the end of 2016.

---

## CFP vs QSFP28: 100G Connectors

[Image showing different connector types: CFP, CFP2, CXP, and QSFP28]

---

## Splitting 100G Ethernet to 25G and 50G

- **100G is actually 4x25g (QSFP28)**, so 100G Ports can be split with "octopus cables" to lower speed.

- **50G (2x25) and 25G (1x25G) speeds are available** which doubles or quadruples the port density of switches.

- **Some switches can handle 32 links of 100G, 64 of 50G and 128 of 25G.**

- **It was a late idea. So 25G Ethernet standards are scheduled to be completed in 2016 only.** Vendors are designing to a proposed standard.

- **50G Ethernet standard is in the works (2018-2020).** May be the default in the future since storage speeds and memory speeds increase.

- **100G Ethernet done**

- **25G Ethernet has a new connector standard called *SFP28***
