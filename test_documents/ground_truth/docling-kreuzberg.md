# Docling Technical Report

Version 1.0 Christoph Auer Maksym Lysak Ahmed Nassar Michele Dolfi Nikolaos Livathinos Panos Vagenas Cesar Berrospi Ramis Matteo Omenetti Fabian Lindlbauer Kasper Dinkla Lokesh Mishra Yusik Kim Shubham Gupta Rafael Teixeira de Lima Valery Weber Lucas Morin Ingmar Meijer Viktor Kuropiatnyk Peter W. J. Staar AI4K Group, IBM Research Ruschlikon, Switzerland ¨

Abstract

This technical report introduces Docling, an easy to use, self-contained, MITlicensed open-source package for PDF document conversion. It is powered bystate-of-the-art specialized AI models for layout analysis (DocLayNet) and tablestructure recognition (TableFormer), and runs efficiently on commodity hardwarein a small resource budget. The code interface allows for easy extensibility andaddition of new features and models.

1 Introduction

Converting PDF documents back into a machine-processable format has been a major challengefor decades due to their huge variability in formats, weak standardization and printing-optimizedcharacteristic, which discards most structural features and metadata. With the advent of LLMsand popular application patterns such as retrieval-augmented generation (RAG), leveraging the richcontent embedded in PDFs has become ever more relevant. In the past decade, several powerfuldocument understanding solutions have emerged on the market, most of which are commercial software, cloud offerings [3] and most recently, multi-modal vision-language models. As of today, only a handful of open-source tools cover PDF conversion, leaving a significant feature and quality gapto proprietary solutions.

With Docling, we open-source a very capable and efficient document conversion tool which buildson the powerful, specialized AI models and datasets for layout analysis and table structure recognition we developed and presented in the recent past [12, 13, 9]. Docling is designed as a simple,self-contained python library with permissive license, running entirely locally on commodity hardware. Its code architecture allows for easy extensibility and addition of new features and models.

Docling Technical Report

Here is what Docling delivers today:

- Converts PDF documents to JSON or Markdown format, stable and lightning fast

- Understands detailed page layout, reading order, locates figures and recovers table structures

- Extracts metadata from the document, such as title, authors, references and language

- Optionally applies OCR, e.g. for scanned PDFs

- Can be configured to be optimal for batch-mode (i.e high throughput, low time-to-solution)or interactive mode (compromise on efficiency, low time-to-solution)

- Can leverage different accelerators (GPU, MPS, etc).

2 Getting Started

To use Docling, you can simply install the docling package from PyPI. Documentation and examplesare available in our GitHub repository at github.com/DS4SD/docling. All required model assets aredownloaded to a local huggingface datasets cache on first use, unless you choose to pre-install themodel assets in advance.

Docling provides an easy code interface to convert PDF documents from file system, URLs or binarystreams, and retrieve the output in either JSON or Markdown format. For convenience, separatemethods are offered to convert single documents or batches of documents. A basic usage exampleis illustrated below. Further examples are available in the Doclign code repository.from docling . document_converter import DocumentConverter source = " https :// arxiv . org/ pdf /2206.01062 " # PDF path or URLconverter = DocumentConverter () result = converter . convert_single ( source ) print ( result . render_as_markdown () ) # output : "## DocLayNet : A Large Human - Annotated Dataset for Document - Layout Analysis [...]" Optionally, you can configure custom pipeline features and runtime options, such as turning on oroff features (e.g. OCR, table structure recognition), enforcing limits on the input document size, anddefining the budget of CPU threads. Advanced usage examples and options are documented in the README file. Docling also provides a Dockerfile to demonstrate how to install and run it inside a container.

3 Processing pipeline

Docling implements a linear pipeline of operations, which execute sequentially on each given document (see Fig. 1). Each document is first parsed by a PDF backend, which retrieves the programmatictext tokens, consisting of string content and its coordinates on the page, and also renders a bitmapimage of each page to support downstream operations. Then, the standard model pipeline applies a sequence of AI models independently on every page in the document to extract features and content,such as layout and table structures. Finally, the results from all pages are aggregated and passedthrough a post-processing stage, which augments metadata, detects the document language, infersreading-order and eventually assembles a typed document object which can be serialized to JSONor Markdown.

3.1 PDF backends

Two basic requirements to process PDF documents in our pipeline are a) to retrieve all text contentand their geometric coordinates on each page and b) to render the visual representation of eachpage as it would appear in a PDF viewer. Both these requirements are encapsulated in Docling’s PDF backend interface. While there are several open-source PDF parsing libraries available forpython, we faced major obstacles with all of them for different reasons, among which were restrictivesee huggingface.co/ds4sd/docling-models/

```
{;}
Assemble results, Serialize as
Apply document JSON
post-processing or Markdown
Parse OCR Layout Table
PDF pages Analysis Structure
Model Pipeline
```

Figure 1: Sketch of Docling’s default processing pipeline. The inner part of the model pipeline iseasily customizable and extensible.

licensing (e.g. pymupdf [7]), poor speed or unrecoverable quality issues, such as merged text cellsacross far-apart text tokens or table columns (pypdfium, PyPDF) [15, 14].

We therefore decided to provide multiple backend choices, and additionally open-source a custombuilt PDF parser, which is based on the low-level qpdf[4] library. It is made available in a separatepackage named docling-parse and powers the default PDF backend in Docling. As an alternative,we provide a PDF backend relying on pypdfium, which may be a safe backup choice in certain cases, e.g. if issues are seen with particular font encodings.

3.2 AI models

As part of Docling, we initially release two highly capable AI models to the open-source community,which have been developed and published recently by our team. The first model is a layout analysismodel, an accurate object-detector for page elements [13]. The second model is TableFormer [12, 9], a state-of-the-art table structure recognition model. We provide the pre-trained weights (hosted onhuggingface) and a separate package for the inference code as docling-ibm-models. Both modelsare also powering the open-access deepsearch-experience, our cloud-native service for knowledgeexploration tasks.

Layout Analysis Model Our layout analysis model is an object-detector which predicts the bounding-boxes and classes ofvarious elements on the image of a given page. Its architecture is derived from RT-DETR [16] andre-trained on DocLayNet [13], our popular human-annotated dataset for document-layout analysis,among other proprietary datasets. For inference, our implementation relies on the onnxruntime [5].

The Docling pipeline feeds page images at 72 dpi resolution, which can be processed on a single CPU with sub-second latency. All predicted bounding-box proposals for document elements arepost-processed to remove overlapping proposals based on confidence and size, and then intersectedwith the text tokens in the PDF to group them into meaningful and complete units such as paragraphs,section titles, list items, captions, figures or tables.

Table Structure Recognition The TableFormer model [12], first published in 2022 and since refined with a custom structure tokenlanguage [9], is a vision-transformer model for table structure recovery. It can predict the logicalrow and column structure of a given table based on an input image, and determine which tablecells belong to column headers, row headers or the table body. Compared to earlier approaches, TableFormer handles many characteristics of tables, such as partial or no borderlines, empty cells,rows or columns, cell spans and hierarchy both on column-heading or row-heading level, tables withinconsistent indentation or alignment and other complexities. For inference, our implementationrelies on PyTorch [2].

The Docling pipeline feeds all table objects detected in the layout analysis to the TableFormer model,by providing an image-crop of the table and the included text cells. TableFormer structure predictions are matched back to the PDF cells in post-processing to avoid expensive re-transcription textin the table image. Typical tables require between 2 and 6 seconds to be processed on a standard CPU, strongly depending on the amount of included table cells.

OCR Docling provides optional support for OCR, for example to cover scanned PDFs or content inbitmaps images embedded on a page. In our initial release, we rely on EasyOCR [1], a popular thirdparty OCR library with support for many languages. Docling, by default, feeds a high-resolutionpage image (216 dpi) to the OCR engine, to allow capturing small print detail in decent quality. While EasyOCR delivers reasonable transcription quality, we observe that it runs fairly slow on CPU (upwards of 30 seconds per page).

We are actively seeking collaboration from the open-source community to extend Docling withadditional OCR backends and speed improvements.

3.3 Assembly

In the final pipeline stage, Docling assembles all prediction results produced on each page into a well-defined datatype that encapsulates a converted document, as defined in the auxiliary packagedocling-core. *The generated document object is passed through a post-processing model whichleverages* several algorithms to augment features, such as detection of the document language, correcting the reading order, matching figures with captions and labelling metadata such as title, authorsand references. The final output can then be serialized to JSON or transformed into a Markdownrepresentation at the users request.

3.4 Extensibility

Docling provides a straight-forward interface to extend its capabilities, namely the model pipeline. A model pipeline constitutes the central part in the processing, following initial document parsingand preceding output assembly, and can be fully customized by sub-classing from an abstract baseclass (BaseModelPipeline) or cloning the default model pipeline. This effectively allows to fullycustomize the chain of models, add or replace models, and introduce additional pipeline configuration parameters. To use a custom model pipeline, the custom pipeline class to instantiate canbe provided as an argument to the main document conversion methods. We invite everyone in thecommunity to propose additional or alternative models and improvements.

Implementations of model classes must satisfy the python Callable interface. The __call__ methodmust accept an iterator over page objects, and produce another iterator over the page objects whichwere augmented with the additional features predicted by the model, by extending the provided PagePredictions data model accordingly.

4 Performance

In this section, we establish some reference numbers for the processing speed of Docling and theresource budget it requires. All tests in this section are run with default options on our standard testset distributed with Docling, which consists of three papers from arXiv and two IBM Redbooks,with a total of 225 pages. Measurements were taken using both available PDF backends on twodifferent hardware systems: one MacBook Pro M3 Max, and one bare-metal server running Ubuntu

20.04 LTS on an Intel Xeon E5-2690 CPU. For reproducibility, we fixed the thread budget (throughsetting OMP NUM THREADS environment variable) once to 4 (Docling default) and once to 16 (equal to full core count on the test hardware). All results are shown in Table 1.

If you need to run Docling in very low-resource environments, please consider configuring thepypdfium backend. While it is faster and more memory efficient than the default docling-parsebackend, it will come at the expense of worse quality results, especially in table structure recovery.

Establishing GPU acceleration support for the AI models is currently work-in-progress and largelyuntested, but may work implicitly when CUDA is available and discovered by the onnxruntime and

torch runtimes backing the Docling pipeline. We will deliver updates on this topic at in a futureversion of this report.

Table 1: Runtime characteristics of Docling with the standard model pipeline and settings, on ourtest dataset of 225 pages, on two different systems. OCR is disabled. We show the time-to-solution (TTS), computed throughput in pages per second, and the peak memory used (resident set size) forboth the Docling-native PDF backend and for the pypdfium backend, using 4 and 16 threads.

CPU Thread budget native backend pypdfium backend TTS Pages/s Mem TTS Pages/s Mem Apple M3 Max 4 177 s 1.27 6.20 GB 103 s 2.18 2.56 GB (16 cores) 16 167 s 1.34 92 s 2.45 Intel(R) Xeon 4 375 s 0.60 6.16 GB 239 s 0.94 2.42 GB E5-2690 16 244 s 0.92 143 s 1.57 (16 cores)

5 Applications

Thanks to the high-quality, richly structured document conversion achieved by Docling, its output qualifies for numerous downstream applications. For example, Docling can provide a basefor detailed enterprise document search, passage retrieval or classification use-cases, or supportknowledge extraction pipelines, allowing specific treatment of different structures in the document,such as tables, figures, section structure or references. For popular generative AI application patterns, such as retrieval-augmented generation (RAG), we provide quackling, an open-source packagewhich capitalizes on Docling’s feature-rich document output to enable document-native optimizedvector embedding and chunking. It plugs in seamlessly with LLM frameworks such as LlamaIndex [8]. Since Docling is fast, stable and cheap to run, it also makes for an excellent choice to builddocument-derived datasets. With its powerful table structure recognition, it provides significant benefit to automated knowledge-base construction [11, 10]. Docling is also integrated within the open IBM data prep kit [6], which implements scalable data transforms to build large-scale multi-modaltraining datasets.

6 Future work and contributions

Docling is designed to allow easy extension of the model library and pipelines. In the future, weplan to extend Docling with several more models, such as a figure-classifier model, an equationrecognition model, a code-recognition model and more. This will help improve the quality of conversion for specific types of content, as well as augment extracted document metadata with additional information. Further investment into testing and optimizing GPU acceleration as well asimproving the Docling-native PDF backend are on our roadmap, too.

We encourage everyone to propose or implement additional features and models, and willgladly take your inputs and contributions under review. The codebase of Docling is open for useand contribution, under the MIT license agreement and in alignment with our contributing guidelinesincluded in the Docling repository. If you use Docling in your projects, please consider citing thistechnical report.

References

[1] J. AI. Easyocr: Ready-to-use ocr with 80+ supported languages. https://github.com/ JaidedAI/EasyOCR, 2024. Version: 1.7.0.

[2] J. Ansel, E. Yang, H. He, N. Gimelshein, A. Jain, M. Voznesensky, B. Bao, P. Bell, D. Berard,

- E. Burovski, G. Chauhan, A. Chourdia, W. Constable, A. Desmaison, Z. DeVito, E. Ellison,

- W. Feng, J. Gong, M. Gschwind, B. Hirsh, S. Huang, K. Kalambarkar, L. Kirsch, M. Lazos, M. Lezcano, Y. Liang, J. Liang, Y. Lu, C. Luk, B. Maher, Y. Pan, C. Puhrsch, M. Reso,

- M. Saroufim, M. Y. Siraichi, H. Suk, M. Suo, P. Tillet, E. Wang, X. Wang, W. Wen, S. Zhang,

- X. Zhao, K. Zhou, R. Zou, A. Mathews, G. Chanan, P. Wu, and S. Chintala. Pytorch 2: Faster

machine learning through dynamic python bytecode transformation and graph compilation. In Proceedings of the 29th ACM International Conference on Architectural Support for Programming *Languages and Operating Systems, Volume 2 (ASPLOS ’24). ACM, 4 2024. doi:*

10.1145/3620665.3640366. URL https://pytorch.org/assets/pytorch2-2.pdf.

[3] C. Auer, M. Dolfi, A. Carvalho, C. B. Ramis, and P. W. Staar. Delivering document conversionas a cloud service with high throughput and responsiveness. In 2022 IEEE 15th International *Conference on Cloud Computing (CLOUD), pages 363–373. IEEE, 2022.*

[4] J. Berkenbilt. Qpdf: A content-preserving pdf document transformer, 2024. URL https:

//github.com/qpdf/qpdf.

[5] O. R. developers. Onnx runtime. https://onnxruntime.ai/, 2024. Version: 1.18.1.

[6] IBM. Data Prep Kit: a community project to democratize and accelerate unstructureddata preparation for LLM app developers, 2024. URL https://github.com/IBM/data-prep-kit.

[7] A. S. Inc. PyMuPDF, 2024. URL https://github.com/pymupdf/PyMuPDF.

[8] J. Liu. LlamaIndex, 11 2022. URL https://github.com/jerryjliu/llama_index.

[9] M. Lysak, A. Nassar, N. Livathinos, C. Auer, and P. Staar. Optimized Table Tokenizationfor Table Structure Recognition. In Document Analysis and Recognition - ICDAR 2023: 17th *International Conference, San Jose, CA, USA, August 21–26, 2023, Proceedings, Part II ´ , pages* 37–50, Berlin, Heidelberg, Aug. 2023. Springer-Verlag. ISBN 978-3-031-41678-1. doi: 10. 1007/978-3-031-41679-8 3. URL https://doi.org/10.1007/978-3-031-41679-8_3.

[10] L. Mishra, S. Dhibi, Y. Kim, C. Berrospi Ramis, S. Gupta, M. Dolfi, and P. Staar. Statements: Universal information extraction from tables with large language models for ESG KPIs. In D. Stammbach, J. Ni, T. Schimanski, K. Dutia, A. Singh, J. Bingler, C. Christiaen, N. Kushwaha, V. Muccione, S. A. Vaghefi, and M. Leippold, editors, Proceedings of the *1st Workshop on Natural Language Processing Meets Climate Change (ClimateNLP 2024),* pages 193–214, Bangkok, Thailand, Aug. 2024. Association for Computational Linguistics. URL https://aclanthology.org/2024.climatenlp-1.15.

[11] L. Morin, V. Weber, G. I. Meijer, F. Yu, and P. W. J. Staar. Patcid: an open-access dataset ofchemical structures in patent documents. Nature Communications, 15(1):6532, August 2024. ISSN 2041-1723. doi: 10.1038/s41467-024-50779-y. URL https://doi.org/10.1038/ s41467-024-50779-y.

[12] A. Nassar, N. Livathinos, M. Lysak, and P. Staar. Tableformer: Table structure understandingwith transformers. In Proceedings of the IEEE/CVF Conference on Computer Vision and *Pattern Recognition, pages 4614–4623, 2022.*

[13] B. Pfitzmann, C. Auer, M. Dolfi, A. S. Nassar, and P. Staar. Doclaynet: a large humanannotated dataset for document-layout segmentation. pages 3743–3751, 2022.

[14] pypdf Maintainers. pypdf: A Pure-Python PDF Library, 2024. URL https://github.com/py-pdf/pypdf.

[15] P. Team. PyPDFium2: Python bindings for PDFium, 2024. URL https://github.com/pypdfium2-team/pypdfium2.

[16] Y. Zhao, W. Lv, S. Xu, J. Wei, G. Wang, Q. Dang, Y. Liu, and J. Chen. Detrs beat yolos onreal-time object detection, 2023.

Appendix

In this section, we illustrate a few examples of Docling’s output in Markdown and JSON.

DocLayNet: A Large Human-Annotated Dataset for DocLayNet: A Large Human-Annotated Dataset for Document-Layout Analysis Document-Layout Analysis

Birgit Pfitzmann Christoph Auer Michele Dolfi

Ahmed S. Nassar Peter Staar

2 2 0 ABSTRACT 2 ABSTRACT n u J ] V .C s c [ v 2 6 0 rms PP) - Te ication (T res Publ CCS CONCEPTS 1 .0 6 Procedu erminal Guide - T rt Users’ FAA Cha 2 : v i X r a CCS CONCEPTS

KEYWORDS KEYWORDS ACM Reference Format:

1 INTRODUCTION

Figure 2: Title page of the DocLayNet paper (arxiv.org/pdf/2206.01062) - left PDF, right rendered Markdown. If recognized, metadata such as authors are appearing first under the title. Text contentinside figures is currently dropped, the caption is retained and linked to the figure in the JSONrepresentation (not shown).

Baselines for Object Detection 5 EXPERIMENTS

Figure 3: Page 6 of the DocLayNet paper. If recognized, metadata such as authors are appearingfirst under the title. Elements recognized as page headers or footers are suppressed in Markdown todeliver uninterrupted content in reading order. Tables are inserted in reading order. The paragraphin ”5. Experiments” wrapping over the column end is broken up in two and interrupted by the table.

KDD ’22, August 14–18, 2022, Washington, DC, USA Birgit Pfitzmann, Christoph Auer, Michele Dolfi, Ahmed S. Nassar, and Peter Staar Table 1: DocLayNet dataset overview. Along with the frequency of each class label, we present the relative occurrence (as % of row “Total”) in the train, test and validation sets. The inter-annotator agreement is computed as the mAP@0.5-0.95 metricbetween pairwise annotations from the triple-annotated pages, from which we obtain accuracy ranges.

A B

% of Total triple inter-annotator mAP @ 0.5-0.95 (%) class label Count Train Test Val All Fin Man Sci Law Pat Ten Caption 22524 2.04 1.77 2.32 84-89 40-61 86-92 94-99 95-99 69-78 n/a Footnote 6318 0.60 0.31 0.58 83-91 n/a 100 62-88 85-94 n/a 82-97 Formula 25027 2.25 1.90 2.96 83-85 n/a n/a 84-87 86-96 n/a n/a List-item 185660 17.19 13.34 15.82 87-88 74-83 90-92 97-97 81-85 75-88 93-95 Page-footer 70878 6.51 5.58 6.00 93-94 88-90 95-96 100 92-97 100 96-98 Page-header 58022 5.10 6.70 5.06 85-89 66-76 90-94 98-100 91-92 97-99 81-86 Picture 45976 4.21 2.78 5.31 69-71 56-59 82-86 69-82 80-95 66-71 59-76 Section-header 142884 12.60 15.77 12.85 83-84 76-81 90-92 94-95 87-94 69-73 78-86 Table 34733 3.20 2.27 3.60 77-81 75-80 83-86 98-99 58-80 79-84 70-85 Text 510377 45.82 49.28 45.00 84-86 81-86 88-93 89-93 87-92 71-79 87-95 Title 5071 0.47 0.30 0.50 60-72 24-63 50-63 94-100 82-96 68-79 24-56 Total 1107470 941123 99816 66531 82-83 71-74 79-81 89-94 86-91 71-76 68-85 include publication repositories such as arXiv3, government offices,company websites as well as data directory services for financial

C reports and patents. Scanned documents were excluded wherever

possible because they can be rotated or skewed. This would notallow us to perform annotation with rectangular bounding-boxesand therefore complicate the annotation process. Preparation work included uploading and parsing the sourced PDF documents in the Corpus Conversion Service (CCS) [22], a cloud-native platform which provides a visual annotation interfaceand allows for dataset inspection and analysis. The annotation interface of CCS is shown in Figure 3. The desired balance of pagesbetween the different document categories was achieved by selective subsampling of pages with certain desired properties. Forexample, we made sure to include the title page of each documentand bias the remaining page selection to those with figures ortables. The latter was achieved by leveraging pre-trained objectdetection models from PubLayNet, which helped us estimate howmany figures and tables a given page contains. Phase 2: Label selection and guideline. We reviewed the collected documents and identified the most common structural features they exhibit. This was achieved by identifying recurrent layoutelements and lead us to the definition of 11 distinct class labels.

Figure 3: Corpus Conversion Service annotation user inter These 11 class labels are Caption, Footnote, Formula, List-item, Page face. The PDF page is shown in the background, with over *footer, Page-header, Picture, Section-header, Table, Text, and Title.* laid text-cells (in darker shades). The annotation boxes can Critical factors that were considered for the choice of these class be drawn by dragging a rectangle over each segment with labels were (1) the overall occurrence of the label, (2) the specificity the respective label from the palette on the right. of the label, (3) recognisability on a single page (i.e. no need for context from previous or next page) and (4) overall coverage of the page. Specificity ensures that the choice of label is not ambiguous, we distributed the annotation workload and performed continuous while coverage ensures that all meaningful items on a page can quality controls. Phase one and two required a small team of experts be annotated. We refrained from class labels that are very specific only. For phases three and four, a group of 40 dedicated annotators to a document category, such as Abstract in the Scientific Articles were assembled and supervised. category. We also avoided class labels that are tightly linked to the Phase 1: Data selection and preparation. Our inclusion cri semantics of the text. Labels such as Author and Affiliation, as seen teria for documents were described in Section 3. A large effort went in DocBank, are often only distinguishable by discriminating on into ensuring that all documents are free to use. The data sources Figure 4: Table 1 from the DocLayNet paper in the original PDF (A), as rendered Markdown (B)

and in JSON representation (C). Spanning table cells, such as the multi-column header ”triple interannotator mAP@0.5-0.95 (%)”, is repeated for each column in the Markdown representation (B), which guarantees that every data point can be traced back to row and column headings only by itsgrid coordinates in the table. In the JSON representation, the span information is reflected in thefields of each table cell (C).

![Image 0 (page 1)](embedded:p1_i0)
