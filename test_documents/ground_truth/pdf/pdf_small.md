# 5-Level Paging and 5-Level EPT

## White Paper

Revision 1.1
May 2017

Document Number: 335252-002

---

## Notice

This document contains information on products in the design phase of development. The information here is subject to change without notice. Do not finalize a design with this information.

Intel technologies' features and benefits depend on system configuration and may require enabled hardware, software, or service activation. Learn more at intel.com, or from the OEM or retailer.

No computer system can be absolutely secure. It does not assume any liability for lost or stolen data or systems or any damages resulting from such issues.

You may not use or facilitate the use of this document in connection with any infringement or other legal analysis concerning Intel products described herein. You agree to grant Intel a non-exclusive, royalty-free license to any patent claim thereafter drafted which includes subject matter disclosed herein.

No license (express or implied, by estoppel or otherwise) to any intellectual property rights is granted by this document.

The products described may contain design defects or errors known as errata which may cause the product to deviate from published specifications. Current characterized errata are available on request.

This document contains information on products, services and/or processes in development. All information provided here is subject to change without notice. Contact your Intel representative to obtain the latest product specifications and roadmaps.

Intel disclaims all express and implied warranties, including without limitation, the implied warranties of merchantability, fitness for a particular purpose, and non-infringement, as well as any warranty arising from course of performance, course of dealing, or usage in trade or commerce.

Copies of documents which have an order number and are referenced in this document may be obtained by calling 1-800-548-4725 or by visiting www.intel.com/design/literature.htm.

Intel, the Intel logo, and Xeon are trademarks of Intel Corporation in the U.S. and/or other countries.

*Other names and brands may be claimed as the property of others.

Copyright © 2016-2017, Intel Corporation. All Rights Reserved.

---

## Contents

### 1 Introduction
1.1 Existing Paging in IA-32e Mode
1.2 Linear-Address Width and VMX Transitions
1.3 Existing Extended Page Tables (EPT)

### 2 Expanding Linear Addresses: 5-Level Paging
2.1 5-Level Paging: Introduction
2.2 Enumeration and Enabling
2.2.1 Enumeration by CPUID
2.2.2 Enabling by Software
2.3 Linear-Address Generation and Canonicality
2.4 5-Level Paging: Linear-Address Translation
2.5 Linear-Address Registers and Canonicality
2.5.1 Canonicality Checking on RIP Loads
2.5.2 Canonicality Checking on Other Loads
2.6 Interactions with TLB-Invalidation Instructions
2.7 Interactions with Intel® MPX
2.8 Interactions with Intel® SGX

### 3 Linear-Address Expansion and VMX Transitions
3.1 Linear-Address Expansion and VM Entries
3.2 Linear-Address Expansion and VM Exits

### 4 5-Level EPT
4.1 5-Level EPT: Guest-Physical-Address Limit
4.2 5-Level EPT: Enumeration and Enabling
4.2.1 Enumeration
4.2.2 Enabling by Software
4.3 5-Level EPT: Guest-Physical-Address Translation
4.4 5-Level EPT and EPTP Switching

### 5 Intel® Virtualization Technology for Directed I/O

## Figures

1-1 Linear-Address Translation Using IA-32e Paging
2-1 Linear-Address Translation Using 5-Level Paging

## Tables

2-1 Format of a PML5 Entry (PML5E) that References a PML4 Table
4-1 Format of an EPT PML5 Entry (EPT PML5E)

---

## Revision History

| Document Number | Revision Number | Description | Date |
|---|---|---|---|
| 335252-001 | 1.0 | Initial Release | November 2016 |
| 335252-002 | 1.1 | Updates to chapter 2, section 2.5.2 "Canonicality Checking on Other Loads". | May 2017 |
