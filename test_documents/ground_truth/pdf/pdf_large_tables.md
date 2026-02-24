# PubTables-1M: Towards Comprehensive Table Extraction from Unstructured Documents

**Brandon Smock, Rohith Pesala, Robin Abraham**

Microsoft, Redmond, WA

brsmock, ropesala, robin.abraham@microsoft.com

## Abstract

Recently, significant progress has been made applying machine learning to the problem of table structure inference and extraction from unstructured documents. However, one of the greatest challenges remains the creation of datasets with complete, unambiguous ground truth at scale. To address this, we develop a new, more comprehensive dataset for table extraction, called PubTables-1M. PubTables-1M contains nearly one million tables from scientific articles, supports multiple input modalities, and contains detailed header and location information for table structures, making it useful for a wide variety of modeling and downstream applications. It also addresses a significant source of ground truth inconsistency observed in prior datasets, called oversegmentation, using a novel canonicalization procedure. We demonstrate that these improvements lead to a significant increase in training performance and a more reliable estimate of model performance at evaluation for table structure recognition. Further, we show that transformer-based object detection methods trained on PubTables-1M produce excellent results for all three tasks of detection, structure recognition, and functional analysis without the any special customization for these tasks.

Data and code will be released at https://github.com/microsoft/table-transformer.

## 1. Introduction

A table is a compact, structured representation for storing data and communicating it in documents and other manners of presentation. In its presented form, however, a table, such as shown in Fig. 1, may not and often does not explicitly represent its logical structure. This is an important problem as a significant amount of data is communicated through documents, but without structure information this data cannot be used in further applications.

The problem of inferring a table's structure from its presentation and converting it to a structured form is known as table extraction (TE). TE entails three subtasks [6], which we illustrate in Fig. 2: table detection (TD), which locates the table; table structure recognition (TSR), which recognizes the structure of a table in terms of rows, columns, and cells; and functional analysis (FA), which recognizes the keys and values of the table. TD is challenging for automated systems [9, 12, 17, 23] due to the wide variety of formats, styles, and structures found in presented tables.

Recently, there has been a shift in the research literature from traditional rule-based methods [4, 11, 18] for TE to data-driven methods based on deep learning (DL) [14, 17, 23]. The primary advantage of DL methods is that they can learn to be more robust to the wide variety of table presentation formats. However, manually annotating tables for TSR is a difficult and time-consuming process [7]. To overcome this, researchers have turned recently to crowd-sourcing to construct larger datasets [9, 23, 23]. These datasets are assembled from tables appearing in documents created by thousands of authors, where an annotation for each table's structure and content is available in a markup format such as HTML, XML, or LaTeX.

While crowd-sourcing solves the problem of dataset size, repurposing annotations originally unintended for TE and automatically converting these to ground truth presents its own set of challenges with respect to completeness, consistency, and quality. This includes not only what information is present but how explicitly this information is represented.

### Figure 1: Example Presentation Table

An example of a presentation table whose underlying structure must be inferred, either manually or by automated systems.

### Table Detection, Structure Recognition, Functional Analysis

The figure illustrates the three subtasks of table extraction:

- **Table Detection** - locates tables in documents
- **Table Structure Recognition** - recognizes rows, columns, and cells
- **Table Functional Analysis** - identifies header cells and data cells

| ASDM | better | equal | Worse | Sum |
|------|--------|-------|-------|-----|
| ASCA better | 1947 (28.9) | 12 (0.02) | 1464 (21.6) | 34,123 (50.8) |
| equal | 1158 (17.2) | 2189 (32.7) | 1024 (1.5) | 24,171 (36.0) |
| ASCA worse | 3755 (0.6) | 2 (0.003) | 5183 (7.7) | 8,940 (13.2) |
| Sum | 24370 (36.2) | 22003 (31.0) | 20861 (31.0) | 67,234 (100.0) |

**Figure 1.** An example of a presentation table whose underlying structure must be inferred, either manually or by automated systems.

## 2. Related Work

### Structure Recognition Datasets

The first dataset to address all three subtasks of table structure recognition was the ICDAR 2013 dataset [6]. It remains popular for benchmarking TSR models due to its quality and relative completeness compared to other datasets. However, as a source of training data for table extraction models is limited, containing only 248 tables for TD and TSR and 192 tables for FA.

Recently, larger datasets [9, 23, 23] have been created by collecting crowd-sourced table annotations automatically from existing documents. We summarize these datasets in Table 1. Each source table has an annotation for its content and structure in a markup format such as HTML, XML, or LaTeX. Various methods are used to determine each table's spatial location within its containing document to create a correspondence between its markup and its presentation.

From there, datasets commonly frame the TSR task as: given an input table, output its structure—the assignment of cells to rows and columns—and the text content for each cell, with image and HTML being example input and output formats, respectively.

More recently, two large datasets, FinTabNet and an enriched version of PubTabNet, have added location information for cells, similar to ICDAR-2013. Adding location information for cells, similar to ICDAR-2013. Adding location information enables the TSR task to be framed as outputting cell bounding boxes for cells defined by these datasets do only identify the text position of each cell whitespace a cell might contain. This has a few implications, such as making bounding boxes for cells defined by these datasets do only identify the text position of each cell whitespace, such as text indentation and alignment. Therefore, one question left open.

### Modeling Approaches

One of the most common modeling approaches for TSR is to frame the task as some form of object detection [14, 17, 23]. Other approaches include those based on image-to-text [16] and graph-based approaches [3, 15]. While a number of general-purpose architectures, such as Faster R-CNN [11] for these model patterns, the unique characteristics of table and the relative lack of training data have both contributed to the commonly observed underperformance of these models when applied to TSR out-of-the-box.

To get around deficiencies in training data, some approaches model TSR in ways that are only partial solutions to the task, such as row and column detection in DeepDesRT [1], which ignores spanning cells or image-to-markup without cell text content, as in models trained on TableBank [9]. Other approaches use custom pipelines that branch to consider different cases separately, such as training separate models to recognize tables with and without visible borders surrounding every cell [14, 23]. Many of the previously mentioned approaches also use engineered model patterns, the unique characteristics of table and the relative lack of training data have both contributed to the commonly observed underperformance of these models when applied to TSR out-of-the-box.

