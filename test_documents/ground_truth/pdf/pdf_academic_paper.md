# A Comprehensive Study of Convergent and Commutative Replicated Data Types

**INRIA**

INSTITUT NATIONAL DE RECHERCHE EN INFORMATIQUE ET EN AUTOMATIQUE

---

## Title Page

A comprehensive study of Convergent and Commutative Replicated Data Types *

**Marc Shapiro**, INRIA & LIP6, Paris, France
**Nuno Preguica**, CITI, Universidade Nova de Lisboa, Portugal
**Carlos Baquero**, Universidade do Minho, Portugal
**Marek Zawirski**, INRIA & UPMC, Paris, France

**N° 7506**

**Janvier 2011**

**Thème COM**

---

Rapport de recherche

---

## Authors and Affiliations

**Marc Shapiro**, INRIA & LIP6, Paris, France
**Nuno Preguica**, CITI, Universidade Nova de Lisboa, Portugal
**Carlos Baquero**, Universidade do Minho, Portugal
**Marek Zawirski**, INRIA & UPMC, Paris, France

Thème COM — Systèmes communicants
Projet Regal

**Rapport de recherche n° 7506 — Janvier 2011 — 47 pages**

---

## Abstract

**Eventual consistency** aims to ensure that replicas of some mutable shared object converge without foreground synchronisation. Previous approaches to eventual consistency are ad-hoc and error-prone. We study a principled approach: to base the design of shared data types on some simple formal conditions that are sufficient to guarantee eventual consistency. We call these types **Convergent or Commutative Replicated Data Types (CRDTs)**. This paper formalises asynchronous object replication, either state based or operation based, and operation based, and provides a sufficient condition appropriate for each case. It describes several useful CRDTs, including container data types supporting both *add* and *remove* operations with clean semantics, and more complex types such as graphs, monotonic DAGs, and sequences. It discusses some properties needed to implement non-trivial CRDTs.

**Key-words:** Data replication, optimistic replication, commutative operations

---

*This research was supported in part by ANR project ConcoRDanT (ANR-10-BLAN 0208), and a Google Research Award 2009. Marek Zawirski is a recipient of the Google Europe Fellowship in Distributed Computing, and this research is supported in part by this Google Fellowship. Carlos Baquero is partially supported by FCT project Castor (PTDC/EIA-EIA/104022/2008).*

---

## Unit Information

**Unité de recherche INRIA Rocquencourt**
Domaine de Voluceau, Rocquencourt, BP 105, 78153 Le Chesnay Cedex (France)
Téléphone : +33 1 39 63 55 11 — Télécopie : +33 1 39 63 53 30

---

## French Abstract

### Étude approfondie des types de données répliqués convergents et commutatifs

**Résumé :** La cohérence à terme vise à assurer que les répliques d'un objet partagé modifiable convergent sans synchronisation à priori. Les approches antérieures du problème sont ad-hoc et sujettes à erreur. Nous proposons une approche basée sur des principes formels : baser la conception des types de données sur des propriétés mathématiques simples, suffisants pour garantir la cohérence à terme. Nous appelons ces types des données des **CRDT (Convergent/Commutative Replicated Data Types)**. Ce papier fournit formalise la réplication asynchrone, qu'elle soit basée sur l'état ou sur les opérations, et fournit une condition suffisante adaptée à chacun de ces cas. Il décrit plusieurs types de CRDT utiles, dont des contenants permettant les opérations *add* et *remove* avec une sémantique propre, et des types de données plus complexes comme les graphes, les graphes acycliques monotones, et les séquences. Il contient une discussion de propriétés dont on a besoin pour mettre en œuvre des CRDT non triviaux.

**Mots-clés :** Réplication des données, réplication optimiste, opérations commutatives

---

## 1. Introduction

Replication is a fundamental concept in distributed systems, well studied by the distributed algorithms community. Much work focuses on maintaining a global total order of operations [24] even in the presence of faults [8]. However, the associated serialisation bottleneck negatively impacts performance and scalability, while the CAP theorem [13] imposes a trade-off between consistency and partition-tolerance.

An alternative approach, *eventual consistency* or *optimistic replication*, is attractive to practitioners [37, 41]. A replica may execute an operation without synchronising a priori with other replicas. The operation is sent asynchronously to other replicas; every replica eventually applies all updates, possibly in different orders. A background consensus algorithm reconciles any conflicting updates [4, 40]. This approach ensures that data remains available despite network partitions. It performs well (as the consensus bottleneck has been moved off the critical path), and the weaker consistency is considered acceptable for some classes of applications. However, reconciliation is generally complex. There is little theoretical guidance on how to design a correct optimistic system, and ad-hoc approaches have proven brittle and error-prone.

In this paper, we study a simple, theoretically sound approach to eventual consistency. We propose the concept of a *convergent or commutative replicated data type (CRDT)*, for which some simple mathematical properties ensure eventual consistency. A trivial example of a CRDT is a replicated counter, which converges because the increment and decrement operations commute (assuming no overflow). Probably, replicas of any CRDT converge to a common state that is equivalent to some correct sequential execution. As a CRDT requires no synchronisation, an update executed immediately, unaffected by network latency, faults, or disconnection. It is extremely scalable and fault-tolerant, and does not require much mechanism. Application areas may include computation in delay-tolerant networks, Internet scale systems, disconnected operation, churn-tolerant peer-to-peer computing, data aggregation, and partition-tolerant cloud computing.

Since, by design, a CRDT does not use consensus, the approach has strong limitations; nonetheless, non-trivial interesting and non-trivial CRDTs are known to exist. For instance, we previously published Treedoc, a sequence CRDT designed for co-operative text editing [32].

Previously, only a handful of CRDTs were known. The objective of this paper is to push the envelope, studying the principles of CRDTs, and presenting a comprehensive portfolio of useful CRDT designs, including variations on registers, counters, sets, graphs, and sequences. We expect them to be of interest to practitioners and theoreticians alike.

Some of our designs suffer from unbounded growth; collecting the garbage requires a weak form of synchronisation [25]. However, its liveness is not essential, as it is an optimisation, off the critical path, and not in the public interface. In the future, we plan to extend the approach to data types where common-case, time-critical operations are commutative.

---

*1 The anomalies of the Amazon Shopping Cart are a well-known example [10].*

RR n° 7506
