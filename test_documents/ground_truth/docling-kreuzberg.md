| 4 |  |
| --- | --- |
| 2 | Docling Technical Report |
| 0 |  |
| 2 |  |
| c |  |
| e |  |
| D | Version 1.0 |
| 9 |  |
| Christoph Panos ] | Auer Maksym Lysak Ahmed Nassar Michele Dolfi Nikolaos Livathinos Vagenas Cesar Berrospi Ramis Matteo Omenetti Fabian Lindlbauer |
| Kasper Dinkla L | Lokesh Mishra Yusik Kim Shubham Gupta Rafael Teixeira de Lima |
| Valery . C | Weber Lucas Morin Ingmar Meijer Viktor Kuropiatnyk Peter W. J. Staar |
| s |  |
| c | AI4K Group, IBM Research |
| [ | Ruschlikon, Switzerland ¨ |
| 5 |  |
| v | Abstract |
| 9 |  |
| 6 |  |
| This 8 | technical report introduces Docling, an easy to use, self-contained, MIT |
| licensed 9 | open-source package for PDF document conversion. It is powered by |
| state-of-the-art structure . 0 | specialized AI models for layout analysis (DocLayNet) and table recognition (TableFormer), and runs efficiently on commodity hardware |
| in 0 8 | a small resource budget. The code interface allows for easy extensibility and |
| addition 4 | of new features and models. |
| 2 |  |
| : |  |
| v |  |
| i |  |
| 1 Introduction X |  |
| r |  |
| Converting a | PDF documents back into a machine-processable format has been a major challenge |
| for decades | due to their huge variability in formats, weak standardization and printing-optimized |
| characteristic, | which discards most structural features and metadata. With the advent of LLMs |
| and popular | application patterns such as retrieval-augmented generation (RAG), leveraging the rich |
| content embedded | in PDFs has become ever more relevant. In the past decade, several powerful |
| document | understanding solutions have emerged on the market, most of which are commercial soft |
| ware, cloud | offerings [3] and most recently, multi-modal vision-language models. As of today, only |
| a handful | of open-source tools cover PDF conversion, leaving a significant feature and quality gap |
| to proprietary | solutions. |
| With Docling, | we open-source a very capable and efficient document conversion tool which builds |
| on the powerful, | specialized AI models and datasets for layout analysis and table structure recog |
| nition we | developed and presented in the recent past [12, 13, 9]. Docling is designed as a simple, |
| self-contained | python library with permissive license, running entirely locally on commodity hard |
| ware. Its code | architecture allows for easy extensibility and addition of new features and models. |
| Docling Technical | Report |
|  | 1 |

## Here is what Docling delivers today:

• Converts PDF documents to JSON or Markdown format, stable and lightning fast

• Understands detailed page layout, reading order, locates figures and recovers table struc
tures

• Extracts metadata from the document, such as title, authors, references and language

## • Optionally applies OCR, e.g. for scanned PDFs

• Can be configured to be optimal for batch-mode (i.e high throughput, low time-to-solution)
or interactive mode (compromise on efficiency, low time-to-solution)

## • Can leverage different accelerators (GPU, MPS, etc).

# 2 Getting Started

To use Docling, you can simply install the *docling* package from PyPI. Documentation and examples

1 are available in our GitHub repository at github.com/DS4SD/docling. All required model assets are downloaded to a local huggingface datasets cache on first use, unless you choose to pre-install the model assets in advance.

Docling provides an easy code interface to convert PDF documents from file system, URLs or binary streams, and retrieve the output in either JSON or Markdown format. For convenience, separate methods are offered to convert single documents or batches of documents. A basic usage example is illustrated below. Further examples are available in the Doclign code repository.

### from docling . document_converter import DocumentConverter

source = " https :// arxiv . org/ pdf /2206.01062 " # PDF path or URL converter = DocumentConverter () result = converter . convert_single ( source ) print ( result . render_as_markdown () ) # output : "## DocLayNet : A Large

### Human - Annotated Dataset for Document - Layout Analysis [...]"

Optionally, you can configure custom pipeline features and runtime options, such as turning on or off features (e.g. OCR, table structure recognition), enforcing limits on the input document size, and defining the budget of CPU threads. Advanced usage examples and options are documented in the README file. Docling also provides a *Dockerfile* to demonstrate how to install and run it inside a container.

# 3 Processing pipeline

Docling implements a linear pipeline of operations, which execute sequentially on each given docu ment (see Fig. 1). Each document is first parsed by a PDF backend, which retrieves the programmatic text tokens, consisting of string content and its coordinates on the page, and also renders a bitmap image of each page to support downstream operations. Then, the standard model pipeline applies a sequence of AI models independently on every page in the document to extract features and content, such as layout and table structures. Finally, the results from all pages are aggregated and passed through a post-processing stage, which augments metadata, detects the document language, infers reading-order and eventually assembles a typed document object which can be serialized to JSON or Markdown.

## 3.1 PDF backends

Two basic requirements to process PDF documents in our pipeline are a) to retrieve all text content and their geometric coordinates on each page and b) to render the visual representation of each page as it would appear in a PDF viewer. Both these requirements are encapsulated in Docling’s PDF backend interface. While there are several open-source PDF parsing libraries available for python, we faced major obstacles with all of them for different reasons, among which were restrictive

### 1 see huggingface.co/ds4sd/docling-models/

## 2

**{;}**

Assemble results, Serialize as Apply document JSON post-processing or Markdown

Parse Layout Table OCR

PDF pages Analysis Structure

Model Pipeline

Figure 1: Sketch of Docling’s default processing pipeline. The inner part of the model pipeline is

## easily customizable and extensible.

licensing (e.g. pymupdf [7]), poor speed or unrecoverable quality issues, such as merged text cells

## across far-apart text tokens or table columns (pypdfium, PyPDF) [15, 14].

We therefore decided to provide multiple backend choices, and additionally open-source a custom

built PDF parser, which is based on the low-level *qpdf[4]* library. It is made available in a separate

package named *docling-parse* and powers the default PDF backend in Docling. As an alternative,

we provide a PDF backend relying on *pypdfium,* which may be a safe backup choice in certain cases,

## e.g. if issues are seen with particular font encodings.

## 3.2 AI models

As part of Docling, we initially release two highly capable AI models to the open-source community,

which have been developed and published recently by our team. The first model is a layout analysis

model, an accurate object-detector for page elements [13]. The second model is TableFormer [12, 9],

a state-of-the-art table structure recognition model. We provide the pre-trained weights (hosted on

huggingface) and a separate package for the inference code as *docling-ibm-models.* Both models

## are also powering the open-access deepsearch-experience, our cloud-native service for knowledge

## exploration tasks.

## Layout Analysis Model

Our layout analysis model is an object-detector which predicts the bounding-boxes and classes of

various elements on the image of a given page. Its architecture is derived from RT-DETR [16] and

## re-trained on DocLayNet [13], our popular human-annotated dataset for document-layout analysis,

among other proprietary datasets. For inference, our implementation relies on the *onnxruntime* [5].

The Docling pipeline feeds page images at 72 dpi resolution, which can be processed on a single

## CPU with sub-second latency. All predicted bounding-box proposals for document elements are

post-processed to remove overlapping proposals based on confidence and size, and then intersected

with the text tokens in the PDF to group them into meaningful and complete units such as paragraphs,

## section titles, list items, captions, figures or tables.

## Table Structure Recognition

The TableFormer model [12], first published in 2022 and since refined with a custom structure token

language [9], is a vision-transformer model for table structure recovery. It can predict the logical

row and column structure of a given table based on an input image, and determine which table

cells belong to column headers, row headers or the table body. Compared to earlier approaches,

TableFormer handles many characteristics of tables, such as partial or no borderlines, empty cells,

rows or columns, cell spans and hierarchy both on column-heading or row-heading level, tables with

## inconsistent indentation or alignment and other complexities. For inference, our implementation

## relies on PyTorch [2].

## 3

The Docling pipeline feeds all table objects detected in the layout analysis to the TableFormer model, by providing an image-crop of the table and the included text cells. TableFormer structure predic tions are matched back to the PDF cells in post-processing to avoid expensive re-transcription text in the table image. Typical tables require between 2 and 6 seconds to be processed on a standard CPU, strongly depending on the amount of included table cells.

## OCR

Docling provides optional support for OCR, for example to cover scanned PDFs or content in bitmaps images embedded on a page. In our initial release, we rely on *EasyOCR* [1], a popular third party OCR library with support for many languages. Docling, by default, feeds a high-resolution page image (216 dpi) to the OCR engine, to allow capturing small print detail in decent quality. While EasyOCR delivers reasonable transcription quality, we observe that it runs fairly slow on CPU (upwards of 30 seconds per page).

We are actively seeking collaboration from the open-source community to extend Docling with additional OCR backends and speed improvements.

## 3.3 Assembly

In the final pipeline stage, Docling assembles all prediction results produced on each page into a well-defined datatype that encapsulates a converted document, as defined in the auxiliary package *docling-core.* The generated document object is passed through a post-processing model which leverages several algorithms to augment features, such as detection of the document language, cor recting the reading order, matching figures with captions and labelling metadata such as title, authors and references. The final output can then be serialized to JSON or transformed into a Markdown representation at the users request.

## 3.4 Extensibility

Docling provides a straight-forward interface to extend its capabilities, namely the model pipeline. A model pipeline constitutes the central part in the processing, following initial document parsing and preceding output assembly, and can be fully customized by sub-classing from an abstract base class *(BaseModelPipeline)* or cloning the default model pipeline. This effectively allows to fully customize the chain of models, add or replace models, and introduce additional pipeline config uration parameters. To use a custom model pipeline, the custom pipeline class to instantiate can be provided as an argument to the main document conversion methods. We invite everyone in the community to propose additional or alternative models and improvements.

Implementations of model classes must satisfy the python Callable interface. The __call__ method must accept an iterator over page objects, and produce another iterator over the page objects which were augmented with the additional features predicted by the model, by extending the provided PagePredictions data model accordingly.

# 4 Performance

In this section, we establish some reference numbers for the processing speed of Docling and the resource budget it requires. All tests in this section are run with default options on our standard test set distributed with Docling, which consists of three papers from arXiv and two IBM Redbooks, with a total of 225 pages. Measurements were taken using both available PDF backends on two different hardware systems: one MacBook Pro M3 Max, and one bare-metal server running Ubuntu 20.04 LTS on an Intel Xeon E5-2690 CPU. For reproducibility, we fixed the thread budget (through setting *OMP NUM THREADS environment variable)* once to 4 (Docling default) and once to 16 (equal to full core count on the test hardware). All results are shown in Table 1.

If you need to run Docling in very low-resource environments, please consider configuring the pypdfium backend. While it is faster and more memory efficient than the default *docling-parse* backend, it will come at the expense of worse quality results, especially in table structure recovery.

Establishing GPU acceleration support for the AI models is currently work-in-progress and largely untested, but may work implicitly when CUDA is available and discovered by the onnxruntime and

## 4

torch runtimes backing the Docling pipeline. We will deliver updates on this topic at in a future

## version of this report.

Table 1: Runtime characteristics of Docling with the standard model pipeline and settings, on our

test dataset of 225 pages, on two different systems. OCR is disabled. We show the time-to-solution

(TTS), computed throughput in pages per second, and the peak memory used (resident set size) for

both the Docling-native PDF backend and for the pypdfium backend, using 4 and 16 threads.

## Thread native backend pypdfium backend

## CPU

## budget TTS Pages/s Mem TTS Pages/s Mem

## Apple M3 Max 4 177 s 1.27 103 s 2.18

## 6.20 GB 2.56 GB

## (16 cores) 16 167 s 1.34 92 s 2.45

## Intel(R) Xeon 4 375 s 0.60 239 s 0.94

## 6.16 GB 2.42 GB

## E5-2690 16 244 s 0.92 143 s 1.57

## (16 cores)

# 5 Applications

Thanks to the high-quality, richly structured document conversion achieved by Docling, its out

put qualifies for numerous downstream applications. For example, Docling can provide a base

## for detailed enterprise document search, passage retrieval or classification use-cases, or support

## knowledge extraction pipelines, allowing specific treatment of different structures in the document,

such as tables, figures, section structure or references. For popular generative AI application pat

## terns, such as retrieval-augmented generation (RAG), we provide quackling, an open-source package

## which capitalizes on Docling’s feature-rich document output to enable document-native optimized

vector embedding and chunking. It plugs in seamlessly with LLM frameworks such as LlamaIn

dex [8]. Since Docling is fast, stable and cheap to run, it also makes for an excellent choice to build

## document-derived datasets. With its powerful table structure recognition, it provides significant ben

efit to automated knowledge-base construction [11, 10]. Docling is also integrated within the open

IBM data prep kit [6], which implements scalable data transforms to build large-scale multi-modal

## training datasets.

# 6 Future work and contributions

Docling is designed to allow easy extension of the model library and pipelines. In the future, we

plan to extend Docling with several more models, such as a figure-classifier model, an equation

recognition model, a code-recognition model and more. This will help improve the quality of con

version for specific types of content, as well as augment extracted document metadata with ad

ditional information. Further investment into testing and optimizing GPU acceleration as well as

## improving the Docling-native PDF backend are on our roadmap, too.

**We encourage everyone to propose or implement additional features and models, and will**

**gladly take your inputs and contributions under review.** The codebase of Docling is open for use

and contribution, under the MIT license agreement and in alignment with our contributing guidelines

included in the Docling repository. If you use Docling in your projects, please consider citing this

## technical report.

# References

## [1] J. AI. Easyocr: Ready-to-use ocr with 80+ supported languages. https://github.com/

## JaidedAI/EasyOCR, 2024. Version: 1.7.0.

[2] J. Ansel, E. Yang, H. He, N. Gimelshein, A. Jain, M. Voznesensky, B. Bao, P. Bell, D. Berard,

E. Burovski, G. Chauhan, A. Chourdia, W. Constable, A. Desmaison, Z. DeVito, E. Ellison,

W. Feng, J. Gong, M. Gschwind, B. Hirsh, S. Huang, K. Kalambarkar, L. Kirsch, M. La

zos, M. Lezcano, Y. Liang, J. Liang, Y. Lu, C. Luk, B. Maher, Y. Pan, C. Puhrsch, M. Reso,

M. Saroufim, M. Y. Siraichi, H. Suk, M. Suo, P. Tillet, E. Wang, X. Wang, W. Wen, S. Zhang,

X. Zhao, K. Zhou, R. Zou, A. Mathews, G. Chanan, P. Wu, and S. Chintala. Pytorch 2: Faster

## 5

machine learning through dynamic python bytecode transformation and graph compilation. In *Proceedings of the 29thA CMI nternational Conference onA rchitectural Supportf or Pro gramming L anguages and Operating Systems, Volume 2 (ASPLOS ’24).* ACM, 4 2024. doi: 10.1145/3620665.3640366. URL https://pytorch.org/assets/pytorch2-2.pdf.

[3] C. Auer, M. Dolfi, A. Carvalho, C. B. Ramis, and P. W. Staar. Delivering document conversion

as a cloud service with high throughput and responsiveness. In *2022I EEE 15thI nternational Conference on Cloud Computing (CLOUD),* pages 363–373. IEEE, 2022.

## [4] J. Berkenbilt. Qpdf: A content-preserving pdf document transformer, 2024. URL https:

## //github.com/qpdf/qpdf.

## [5] O. R. developers. Onnx runtime. https://onnxruntime.ai/, 2024. Version: 1.18.1.

[6] IBM. Data Prep Kit: a community project to democratize and accelerate unstructured

## data preparation for LLM app developers, 2024. URL https://github.com/IBM/ data-prep-kit.

## [7] A. S. Inc. PyMuPDF, 2024. URL https://github.com/pymupdf/PyMuPDF.

## [8] J. Liu. LlamaIndex, 11 2022. URL https://github.com/jerryjliu/llama_index.

[9] M. Lysak, A. Nassar, N. Livathinos, C. Auer, and P. Staar. Optimized Table Tokenization

for Table Structure Recognition. In *DocumentA nalysis andR ecognition -I CDAR 2023: 17th International Conference, SanJ ose,* pages´ *CA, USA,A ugust 21–26, 2023, Proceedings, PartI I* , 37–50, Berlin, Heidelberg, Aug. 2023. Springer-Verlag. ISBN 978-3-031-41678-1. doi: 10. https://doi.org/10.1007/978-3-031-41679-8_3.1007/978-3-031-41679-8 3. URL

[10] L. Mishra, S. Dhibi, Y. Kim, C. Berrospi Ramis, S. Gupta, M. Dolfi, and P. Staar. State

ments: Universal information extraction from tables with large language models for ESG KPIs. In D. Stammbach, J. Ni, T. Schimanski, K. Dutia, A. Singh, J. Bingler, C. Christi aen, N. Kushwaha, V. Muccione, S. A. Vaghefi, and M. Leippold, editors, *Proceedings of the 1st Workshop on N atural L anguage Processing M eets Climate Change (ClimateNLP 2024),* pages 193–214, Bangkok, Thailand, Aug. 2024. Association for Computational Linguistics. URL https://aclanthology.org/2024.climatenlp-1.15.

[11] L. Morin, V. Weber, G. I. Meijer, F. Yu, and P. W. J. Staar. Patcid: an open-access dataset of

chemical structures in patent documents. *Nature Communications,* 15(1):6532, August 2024. ISSN 2041-1723. doi: 10.1038/s41467-024-50779-y. URL https://doi.org/10.1038/ s41467-024-50779-y.

[12] A. Nassar, N. Livathinos, M. Lysak, and P. Staar. Tableformer: Table structure understanding

with transformers. In *Proceedings of the I EEE/CVF Conference on Computer Vision and PatternR ecognition,* pages 4614–4623, 2022.

[13] B. Pfitzmann, C. Auer, M. Dolfi, A. S. Nassar, and P. Staar. Doclaynet: a large human

## annotated dataset for document-layout segmentation. pages 3743–3751, 2022.

## [14] pypdf Maintainers. pypdf: A Pure-Python PDF Library, 2024. URL https://github.com/

## py-pdf/pypdf.

## [15] P. Team. PyPDFium2: Python bindings for PDFium, 2024. URL https://github.com/

## pypdfium2-team/pypdfium2.

[16] Y. Zhao, W. Lv, S. Xu, J. Wei, G. Wang, Q. Dang, Y. Liu, and J. Chen. Detrs beat yolos on

## real-time object detection, 2023.

## 6

# Appendix

In this section, we illustrate a few examples of Docling’s output in Markdown and JSON.

**DocLayNet: A Large Human-Annotated Dataset for Document-Layout Analysis**

DocLayNet: A L arge Human-Annotated Dataset for

Document-Layout A nalysis

Birgit Pfitzmann IBM Research Rueschlikon, Switzerland bpf@zurich.ibm.com

Christoph Auer IBM Research Rueschlikon, Switzerland cau@zurich.ibm.com

Birgit P !tzmann Christoph A uer Michele Dol!

IBM Research IBM R esearch IBM R esearch

Michele Dolfi IBM Research Rueschlikon, Switzerland dol@zurich.ibm.com

Rueschlikon, Switzerland Rueschlikon, S witzerland Rueschlikon, S witzerland

bpf@zurich.ibm.com cau@zurich.ibm.com dol@zurich.ibm.com

Ahmed S. Nassar IBM Research Rueschlikon, Switzerland ahn@zurich.ibm.com

Ahmed S. Nassar Peter S taar

Peter Staar IBM Research Rueschlikon, Switzerland taa@zurich.ibm.com

IBM Research IBM Research

Rueschlikon, Switzerland Rueschlikon, Switzerland

2

ABSTRACTahn@zurich.ibm.com taa@zurich.ibm.com

2

0

ABSTRACT markings3 signs, signals and road

**cont.2 AY11236OPERATION ( ) MODEL**

groundInterpupillary Slide Adjustment Accurate document layout analysis is a key requirement for highquality PDF document conversion. With the recent availability of public, large

**Model AY11230 Model AY11234** Eyepiece

**SELECTING HeadMAGNIFICATION OBJECTIVE FOCUSING** 1. Turn the focusing knob away or toward Rotating In chapter 2, you and your vehicle, you learned about

viewed.Accurate 1. There are two objectives. The lower you until a clear image is in this chapter some of the controls in your vehicle. This chapter is a handy

document layout analysis is a key requirement for high magnification objective has a greater 2. If the image is unclear, adjust the Revolving Turret reference section that gives examples of the most common

again.n While2. In depth order of to field observe and view. the specimen height then turn of the the elevator focusing up knob or down, SignsStand • signs, signals and road markings that keep traffic organized truth datasets such as PubLayNet and DocBank, deep-learning models have proven to be very effective at layout detection and segmentation.

easily use the lower magnification –r egulatory signs and flowing smoothly. objective first. Then, by rotating the **ZOOM MAGNIFICATION** Objectives –s chool, case, the magnification can be 1. Turn the zoom magnification knob to playground and

quality PDF d ocument c onversion. W ith t he recent availability o f changed. the view. desired magnification and field of SignsStage Coarse crosswalk signs

u **CHANGING DISTANCE THE INTERPUPILLARY** 2. In that most you situations, focus at the it is lowest recommended Condenser signsKnobFocusing Knob Adjustment Fine –l ane use There are three ways to read signs: by their shape, colour and

1. The distance between the observer's magnification, then move to a higher Lamp Adjustment –t urn control signs articlethe messages printed on them. Understanding these three ways these datasets are of adequate size to train such models, they severely lack in layout variability since they are sourced from scientific
ClipSwitch2. pupils is the interpupillary distance. magnification and re-focus as KnobOn/Off Stage –p arking signs of classifying signs will help you figure out the meaning of signs

ocBank,J public, large g round-truth d atasets such as P ubLayNet a nd D To rotate adjust the the prism interpupillary caps until both distance eyes 3. If necessary. the image is not clear to both eyes Adjustment –r signs eserved lane that are new to you.

coincide with the image in the at the same time, the diopter ring may Power eyepiece. need adjustment. Cord –w arning signs

**markersFOCUSING** 1. Remove the lens protectivec over. **DIOPTER** 1. To adjust **RING** the eyepiece **ADJUSTMENT** for viewing with Lamp –o are–c onstruction bject repositories such as PubMed and arXiv only. Consequently, the accuracy of the layout segmentation drops significantly when these models

2. Place the specimen on the working or without eyeglasses and for signs

deep-learning models have proven to be very e"ective at layout stage. differences in acuity between the right **Model AY11236** –i nformation and

steps:2 3. Focus first while the specimen turning the with focus the knob left eye until and left eyes, follow the following right-of-waydestination signs Stop Yield the

the image appearsc lear and sharp. a. Observe an image through the left **signsMICROSCOPE** –r ailway **USAGE4.** Rotate the right eyepiece ring until the eyepiece and bring a specific point

images are sharp in and each clear. eyepiece coincide and b. By into turning focusu the sing diopter the focus ring knob. • Signals applied on more challenging and diverse layouts. In this paper, we present DocLayNet , a new, publicly available, document-layout annotation dataset

compounddetection and segmentation. While these d atasets are of adequate adjustment for the left eyepiece, BARSKA Model AY11236 is a powerful fixed power –l ane control

specimenCHANGING **THE BULB** bring the same point into sharp signalsmicroscope designed for biological studies such as 1. Disconnect the power cord. focus. lightsexamination. It can also be used for examining bacteria and –t raffic

2. When oblique the illuminator bulb isc ool, cap remove and remove the c.Then focus bring through the the same right point eyepiece into for general clinical and medical studies and other scientific uses. • Road markings School zone signs

theh alogen bulb withc ap. by turning the right diopter ring. –y ellow lines Shows driving Explains lane use are fluorescent

] **linesCONSTRUCTIONsize** to train such models, they severely lack in layout variability 3. regulations4. Replace Open the with window a new in halogen the base bulb. plate and d.With viewer more should than note one their viewer, own each PDF–w hite yellow-green in COCO format. It contains 80863 manually annotated pages from diverse data sources to represent a wide variability in layouts. For each

replace the halogen lamp or diopter ring position for the left –r eserved lane fluorescent lamp of transmitted and right eyepieces, then before markingsBARSKA Model AY11236 is a fixed power compound microscope.

illuminator. viewing adjustments set the to that diopter setting. ring markingsIt is constructed with two optical paths at the same angle. It is –o ther

instrument,USING **THE VERTICAL TUBE -** equipped with transmitted illumination. By using this

fineV tosince they are sourced from scienti!c article repositories such as **BULBMODELS AY11230/11234 CHANGING** 1. Disconnect **THE** the power cord from the the and1000x user by can selecting observe the specimens desired objective at magnification lens. Coarse from and 40x page, the layout annotations provide labelled bounding-boxes with a choice of 11 distinct classes. DocLayNet also provides a subset of double-

outlet.1. The vertical tube can be used for electrical instructional viewing or to photograph rotating2. When the bulb is cool, remove the focus adjustments provide accuracy and image detail. The Tells about motorist Shows a permitted Shows an action that micro the image TV unit with a digital camera or a oblique the halogen illuminator bulb with cap cap. and remove permittedhead allows the user to position the eyepieces for maximum actionservices is not

2. Loosen the retention screw, then rotate 3. Replace with a new halogen bulb. viewing comfort and easy access to all adjustment knobs.

PubMed and a rXiv o nly. C onsequently, the accuracy of the l ayout length the adjustment of the vertical ring to tube. change the 4. Open the window in the base plate

3. Make sure that botht he images in and fluorescent replace lamp the halogen of transmitted lamp or triple-annotated pages to determine the inter-annotator agreement. In multiple experiments, we provide baseline accuracy scores (in mAP) for a set

illuminator.C

**1413** Shows distance and

. directionsegmentation drops signi!cantly w hen these models are a pplied Warns of hazards Warns of Railway crossing

ahead construction zones

agreement.29s of popular object detection models. We also demonstrate that these models fall approximately 10% behind the inter-annotator

on more challenging and diverse layouts. I n t his p aper, we present

c AGL Energy LimitedA BN 74 115 061 375 Furthermore, we provide evidence that DocLayNet is of sufficient size. Lastly, we compare models trained on PubLayNet, DocBank and DocLayNet,

annotation[ *DocLayNet,* a new, publicly available, document-layout

**MinimumsAGL 2013 Financial Calendar Circling**

22 August 2012 2012 full year result and final dividend announced 7KHUHZDVDFKDQJHWRWKH7(536FULWHULDLQ WKDWDႇHFWVFLUFOLQJDUHDGLPHQVLRQE\H[SDQGLQJWKHDUHDVWRSURYLGH

30 August 2012 Ex-dividend trading commences document-layoutimproved obstacle protection. To indicate that the new criteria had been applied to a given procedure, a is placed on showing that layout predictions of the DocLayNettrained models are more robust and thus the preferred choice for general-purpose

dividenddataset in COCO format. It contains 80863 manually annotated 5 September 2012 Record date for 2012 final the circling line of minimums. The new circling tables and explanatory information is located in the Legend of the TPP.

27 September 2012 Final dividend payable

23 October 2012 Annual General Meeting 7KHDSSURDFKHVXVLQJVWDQGDUGFLUFOLQJDSSURDFKDUHDVFDQEHLGHQWL¿HGE\WKHDEVHQFHRIWKH on the circling line of announced27 February 2013 1 2013 interim result and interim dividend minima.

1 analysis.pages 28 August 2013 1 2013 full year results and final dividend announced

from diverse data sources to represent a wide variability in 1 Indicative dates only, subject to change/Board confirmation

AGL’s Annual General Meeting will be held at the City Recital Hall, Angel Place, Sydney

2012.v commencing at 10.30am on Tuesday 23 October

*$SSO\([SDQGHG&LUFOLQJ$SSURDFK0DQHXYHULQJ$LUVSDFH5DGLXVlayouts.* F or each P DF p age, t he layout annotations p rovide l abelled *Table*

*$SSO\6WDQGDUG&LUFOLQJ$SSURDFK0DQHXYHULQJ5DGLXV7DEOH*

2

SKETCHbounding-boxes with a choice of 11 distinct classes. DocLayNet s **AIRPORT**

mre T - The airport sketch is a depiction of the airport with emphasis on runway pattern and related

)6 **Looking back on** piP information, positioned in either the lower left or lower right corner of the chart to aid

**CONCEPTSP** T( lot recognition of the airport from the air and to provide some information to aid on ground **CCS**

nalso provides a subset of double- and triple-annotated pages to oit navigation of the airport. The runways are drawn to scale and oriented to true north. Runway

runways.acilb0 dimensions (length and width) are shown for all active

**175 years of** uP

Runway(s) are depicted based on what type and construction of the runway.

seruddetermine the inter-annotator a greement. I n m ultiple experiments, e Constructioncor Hard Surface Other Than Metal Surface Closed Runway Under

1 **llooking forward.** SurfaceP Hard

animreT

;we p rovide b aseline a ccuracy scores ( in m AP) f or a s et of p opular - Runwayedi Stopways, Displaced Closed Water · Information systems → Document structure ; · Applied computing → Document analysis ; · Computing methodologies → Machine learning

Pavementu0 Taxiways, Park Threshold Areas’sres. G ing

Yesterday U

trobject detection models. We also demonstrate that these models ah

;Established in Sydney in 1837, and then C Computer vision ; Object detection

Aknowna dimenFthe s The Australian Gas Light Company, A Taxiways and aprons are shaded grey. Other runway features that may be shown are runway numbers, runway

threshold.6 AGL business has an established history sions, runway slope, arresting gear, and displaced

andr eputation for serving the gas and

fall approximately 10% behind the inter-annotator agreement. Fur electricity KHOLIn1 841, whenA needs GL of supplied Australian the households. gas to light 2WKHULQIRUPDWLRQFRQFHUQLQJOLJKWLQJ ¿QDODSSURDFKEHDULQJV DLUSRUWEHDFRQ REVWDFOHV FRQWUROWRZHU 1$9$,'V

shown.thefi rst public street lamp, it was reported pads may also be

0 in theS ydney Gazette as a “wonderful

achievement of scientific knowledge, assisted **$LUSRUW(OHYDWLRQDQG7RXFKGRZQ=RQH(OHYDWLRQ** bym echanical ingenuity.” Within two years,

thermore, we provide evidence that D ocLayNet i s o f s u#cient size. 165g as lamps were lighting the City of Sydney. The airport elevation is shown enclosed within a box in the upper left corner of the sketch box and the touchdown zone Permission to make digital or hard copies of part or all of this work for personal or classroom use is granted without fee provided that copies are not

elevation (TDZE) is shown in the upper right corner of the sketch box. The airport elevation is the highest point of an

IHHWRI2 DLUSRUW¶VXVDEOHUXQZD\VPHDVXUHGLQIHHWIURPPHDQVHDOHYHO 7KH7'=(LVWKHKLJKHVWHOHYDWLRQLQWKH¿UVW

the landing surface. Circling only approaches will not show a TDZE.

Lastly, we compare models trained on PubLayNet, DocBank and

made or distributed for profit or commercial advantage and that copies bear this notice and the full citation on the first page. Copyrights for third-party

2

: DocLayNet, showing that layout predictions of the DocLayNet

components of this work must be honored. For all other uses, contact the owner/author(s).

114v

trained models are more robust and thus the preferred choice for

i

general-purpose document-layout analysis.

KDD '22, August 14-18, 2022, Washington, DC, USA © 2022 Copyright held by the owner/author(s). ACM ISBN 978-1-4503-9385-0/22/08.

X

r Figure 1: Four examples of complex page layouts across dif

https://doi.org/10.1145/3534678.3539043

a

ategoriesCCS CONCEPTS ferent document c

Figure 1: Four examples of complex page layouts across different document categories

• Information systems → Document structure; • Applied com

puting → Document analysis; • Computing methodologies

→ Machine learning; Computer vision; *Object d etection;*

KEYWORDS **KEYWORDS**

PDF document c onversion, layout segmentation, object-detection,

data set, Machine L earning PDF document conversion, layout segmentation, object-detection, data set, Machine Learning

Permission to make digital or hard copies of part or all of this work for personal or

ACM Reference Format:

classroom use is granted without fee provided that copies are not made or distributed

for pro!t o r commercial advantage a nd t hat copies b ear t his notice a nd the f ull c itation Birgit P!tzmann, C hristoph A uer, Michele Dol!, Ahmed S. Nassar, and Peter **ACM Reference Format:**

on the !rst page. Copyrights for third-party components of this work must be honored.

Staar. 2022. DocLayNet: A Large Human-Annotated Dataset for Document

For all other uses, c ontact the owner/author(s).

Layout Analysis. In *Proceedings of the 28th ACM SIGKDD Conference on*

*KDD ’22, August 14–18, 2022, Washington, DC, USA*

Birgit Pfitzmann, Christoph Auer, Michele Dolfi, Ahmed S. Nassar, and Peter Staar. 2022. DocLayNet: A Large Human-Annotated Dataset for

*Knowledge Discovery and Data Mining (KDD ’22), August 14–18, 2022, Wash*

© 2022 Copyright held b y the owner/author(s).

https://doi.org/10.1145/ACM *14-18,ington, DC, USA.* ACM, N ew York, N Y, U SA, 9 p ages. DocumentLayout Analysis. In Proceedings of the 28th ACM SIGKDD Conference on Knowledge Discovery and Data Mining (KDD '22), August ISBN 978-1-4503-9385-0/22/08.

https://doi.org/10.1145/3534678.3539043 3534678.3539043

2022, Washington, DC, USA. ACM, New York, NY, USA, 9 pages. https://doi.org/10.1145/ 3534678.3539043

**1 INTRODUCTION**

Despite the substantial improvements achieved with machine-learning (ML) approaches and deep neural networks in recent years, document

Figure 2: Title page of the DocLayNet paper (arxiv.org/pdf/2206.01062) - left PDF, right rendered

conversion remains a challenging problem, as demonstrated by the numerous public competitions held on this topic [1-4]. The challenge originates

from the huge variability in PDF documents regarding layout, language and formats (scanned, programmatic or a combination of both). Engineering a

Markdown. If recognized, metadata such as authors are contentsingle appearing ML model that can be applied first on all types under of documents and the provides high-quality title. layout Text segmentation remains to this day extremely

challenging [5]. To highlight the variability in document layouts, we show a few example documents from the DocLayNet dataset in Figure 1.

inside figures is currently dropped, the caption is retained and linked to the figure in the JSON

## representation (not shown).

## 7

KDD ’22, August 14–18, 2022, Washington, DC, USA Birgit Pfitzmann, Christoph Auer, Michele Dolfi, Ahmed S. Nassar, and Peter Staar

Table 2: Prediction performance (mAP@0.5-0.95) of object

detection networks on DocLayNet test set. The MRCNN

(Mask R-CNN) and FRCNN (Faster R-CNN) models with

ResNet-50 or ResNet-101 backbone were trained based on

the network architectures from the *detectron2* model zoo

(Mask R-CNN R50, R101-FPN 3x, Faster R-CNN R101-FPN

3x), with default con!gurations. The YOLO implementation

utilized was YOLOv5x6 [13]. All models were initialised us

ing pre-trained weights from the COCO 2017 dataset.

human MRCNN FRCNN YOLO

R50 R101 R101 v5x6

Caption 84-89 68.4 71.5 70.1 77.7

Footnote 83-91 70.9 71.8 73.7 77.2

Formula 83-85 60.1 63.4 63.5 66.2

List-item 87-88 81.2 80.8 81.0 86.2

Page-footer 93-94 61.6 59.3 58.9 61.1

Page-header 85-89 71.9 70.0 72.0 67.9

Figure 5: Prediction performance (mAP@0.5-0.95) of a Mask

Picture 69-71 71.7 72.7 72.0 77.1

R-CNN network with ResNet50 backbone trained on increas

Section-header 83-84 67.6 69.3 68.4 74.6

ing fractions of the DocLayNet dataset. The learning curve

Table 77-81 82.2 82.9 82.2 86.3

"attens around the 80% mark, indicating that increasing the

Text 84-86 84.6 85.8 85.4 88.1

size of the DocLayNet dataset with similar data will not yield

Title 60-72 76.7 80.4 79.9 82.7

signi!cantly b etter predictions.

All 82-83 72.4 73.5 73.4 76.8

to a void t his a t any cost i n o rder to h ave clear, unbiased b aseline

numbers f or human document-layout annotation. T hird, w e in

paper and l eave the detailed evaluation o f more r ecent methods

troduced the feature of *snapping* boxes a round t ext s egments t o

mentioned in Section 2 for future work.

obtain a pixel-accurate annotation and again reduce time and e!ort.

In this s ection, we will present several aspects related to the

The C CS a nnotation t ool a utomatically s hrinks e very u ser-drawn

performance of object detection models on DocLayNet. Similarly

box t o the minimum bounding-box around the enclosed text-cells

as i n P ubLayNet, w e w ill evaluate the quality o f their predictions

for all purely text-based segments, which excludes only *Table* and

using m ean average p recision (mAP) w ith 1 0 o verlaps that r ange

*Picture.* For the latter, we instructed annotation sta! to m inimise

from 0.5 t o 0.95 in steps of 0.05 (mAP@0.5-0.95). These scores are

inclusion o f s urrounding w hitespace w hile i ncluding a ll g raphical

computed by leveraging the evaluation code provided by the COCO

lines. A d ownside o f snapping b oxes to e nclosed t ext c ells is t hat

API [16].

some wrongly parsed P DF pages cannot be annotated correctly and

need to be skipped. Fourth, we established a way to "ag pages as

Baselines for Object Detection

*rejected* for cases where no valid annotation according to the label

In Table 2, we present baseline experiments (given in mAP) on Mask guidelines could be achieved. Example cases for this would be PDF

R-CNN [12], F aster R -CNN [ 11], and YOLOv5 [13]. Both t raining pages that render incorrectly or contain layouts t hat are impossible

and evaluation were performed on RGB images with dimensions of to capture with non-overlapping rectangles. Such rejected pages are

1025 ⇥ 1025 pixels. For training, we only used one annotation in case not contained in the #nal dataset. W ith a ll t hese m easures in p lace,

of redundantly annotated pages. As one can observe, the variation experienced annotation sta! managed to annotate a single page in

in m AP b etween the models i s r ather low, but overall b etween 6 a typical timeframe of 20s to 60s, depending on its complexity.

and 10% lower than the mAP computed from the pairwise human

indication5 EXPERIMENTS annotations on triple-annotated pages. This gives a good

that the D ocLayNet dataset poses a worthwhile challenge for the

research community to close the gap between human recognition

The p rimary goal of DocLayNet is to obtain h igh-quality ML models

and ML approaches. It is interesting to see that Mask R-CNN and

capable of a ccurate d ocument-layout a nalysis o n a wide v ariety

Faster R -CNN produce very c omparable mAP scores, indicating

of c hallenging l ayouts. A s d iscussed i n S ection 2 , o bject detection

that pixel-based image segmentation derived from bounding-boxes

models are currently the e asiest to u se, due to the standardisation

does not help to obtain better predictions. On the other hand, the

of ground-truth data in COCO format [16] and the availability of

more r ecent Yolov5x model does very well and even out-performs

general frameworks s uch as *detectron2* [17]. Furthermore, baseline

humans o n s elected l abels such a s *Text, Table* and *Picture.* This is

numbers in PubLayNet and DocBank were obtained using standard

not entirely surprising, as *Text, Table* and *Picture* are abundant and

object detection models such as Mask R -CNN a nd F aster R-CNN.

the most visually distinctive in a document.

As such, w e w ill relate to these object detection methods in this

Figure 3: Page 6 of the DocLayNet paper. If recognized, metadata such as authors are appearing

first under the title. Elements recognized as page headers or footers are suppressed in Markdown to

deliver uninterrupted content in reading order. Tables are inserted in reading order. The paragraph

in ”5. Experiments” wrapping over the column end is broken up in two and interrupted by the table.

## 8

KDD ’ 22, A ugust 1 4–18, 2 022, W ashington, DC, USA Birgit P fitzmann, Christoph A uer, Michele Dolfi, Ahmed S. Nassar, and Peter Staar

Table 1: DocLayNet dataset overview. Along with the f requency of e ach class label, we present the relative occurrence (as %

of row “Total”) i n the train, test and v alidation sets. T he i nter-annotator a greement is computed a s the mAP@0.5-0.95 metric

thebetween p airwise annotations from t he triple-annotated p ages, from which w e obtain accuracy r anges. Table 1: DocLayNet dataset overview. Along with the frequency of each class label, we present the relative occurrence (as % of row "Total") in

train, test and validation sets. The inter-annotator agreement is computed as the mAP@0.5-0.95 metric between pairwise annotations from the triple

B

A

annotated pages, from which we obtain accuracy ranges.

% o f Total triple i nter-annotator mAP @ 0 .5-0.95 ( %)

**triple triple triple triple triple triple triple**

class label Count Train Test Val All Fin Man Sci Law Pat Ten **inter inter inter inter inter inter inter**

**% of % of % of annotator annotator annotator annotator annotator annotator annotator**

Caption 22524 2.04 1.77 2.32 84-89 40-61 86-92 94-99 95-99 69-78 n/a

**Total Total Total mAP @ mAP @ mAP @ mAP @ mAP @ mAP @ mAP @**

Footnote 6318 0.60 0.31 0.58 83-91 n/a 100 62-88 85-94 n/a 82-97

**0.5-0.95 0.5-0.95 0.5-0.95 0.5-0.95 0.5-0.95 0.5-0.95 0.5-0.95**

Formula 25027 2.25 1.90 2.96 83-85 n/a n/a 84-87 86-96 n/a n/a

**(%) (%) (%) (%) (%) (%) (%)**

List-item 185660 17.19 13.34 15.82 87-88 74-83 90-92 97-97 81-85 75-88 93-95

class

TenPage-footer 70878 6.51 5.58 6.00 93-94 88-90 95-96 100 92-97 100 96-98 Count Train Test Val All Fin Man Sci Law Pat

label

Page-header 58022 5.10 6.70 5.06 85-89 66-76 90-94 98-100 91-92 97-99 81-86

Caption 22524 2.04 1.77 2.32 84-89 40-61 86-92 94-99 95-99 69-78 n/a

Picture 45976 4.21 2.78 5.31 69-71 56-59 82-86 69-82 80-95 66-71 59-76

82-97Section-header 142884 12.60 15.77 12.85 83-84 76-81 90-92 94-95 87-94 69-73 78-86 Footnote 6318 0.60 0.31 0.58 83-91 n/a 100 62-88 85-94 n/a

Table 34733 3.20 2.27 3.60 77-81 75-80 83-86 98-99 58-80 79-84 70-85

Formula 25027 2.25 1.90 2.96 83-85 n/a n/a 84-87 86-96 n/a n/a

Text 510377 45.82 49.28 45.00 84-86 81-86 88-93 89-93 87-92 71-79 87-95

List-item 185660 17.19 13.34 15.82 87-88 74-83 90-92 97-97 81-85 75-88 93-95

Title 5071 0.47 0.30 0.50 60-72 24-63 50-63 94-100 82-96 68-79 24-56

Page

Total 1107470 941123 9 9816 6 6531 82-83 71-74 79-81 89-94 86-91 71-76 68-85

70878 6.51 5.58 6.00 93-94 88-90 95-96 100 92-97 100 96-98

footer

Page

81-863 58022 5.10 6.70 5.06 85-89 66-76 90-94 98-100 91-92 97-99

include p ublication r epositories such as arXiv , government o"ces, header

company websites as well as data directory services for #nancial

Picture 45976 4.21 2.78 5.31 69-71 56-59 82-86 69-82 80-95 66-71 59-76

reports and patents. Scanned documents were excluded wherever

C

Section

possible because they can be rotated or skewed. This would not 142884 12.60 15.77 12.85 83-84 76-81 90-92 94-95 87-94 69-73 78-86

header

allow us to perform annotation with rectangular bounding-boxes

Table 34733 3.20 2.27 3.60 77-81 75-80 83-86 98-99 58-80 79-84 70-85

and therefore complicate the annotation process.

Preparation w ork i ncluded u ploading a nd parsing the sourced Text 510377 45.82 49.28 45.00 84-86 81-86 88-93 89-93 87-92 71-79 87-95

PDF documents in the Corpus Conversion Service (CCS) [22], a

Title 5071 0.47 0.30 0.50 60-72 24-63 50-63 94-100 82-96 68-79 24-56

cloud-native platform which provides a v isual annotation i nterface

Total 1107470 941123 99816 66531 82-83 71-74 79-81 89-94 86-91 71-76 68-85

and a llows f or dataset inspection and analysis. The annotation in

terface of C CS is shown in Figure 3. The desired balance of pages

between the di!erent document categories was achieved b y se

lective subsampling of pages with certain desired properties. For

example, we made sure to include the title page of each document

and bias the remaining page selection to those with #gures or

tables. The latter was achieved by leveraging pre-trained object

detection models from PubLayNet, which h elped us estimate how

many #gures and tables a given page contains.

Phase 2: Label selection and guideline. We reviewed the col

lected documents a nd identi#ed the most c ommon structural fea

tures t hey e xhibit. This w as achieved by i dentifying recurrent layout

elements and lead us to the de#nition of 11 distinct class labels.

PageFigure 3 : Corpus C onversion S ervice a nnotation user inter These 11 class l abels are *Caption, Footnote, Formula, List-item,*

Title.face. The P DF page is shown in t he b ackground, w ith o ver *footer, Page-header, Picture, Section-header, Table, Text,* and

classlaid t ext-cells ( in d arker shades). The annotation boxes can Critical factors that were considered for the choice of these

speci#citybe d rawn b y d ragging a rectangle o ver each s egment w ith labels were ( 1) t he overall o ccurrence of the label, (2) the

forthe r espective l abel from the p alette on t he right. of the label, (3) recognisability on a single page (i.e. no need

context from previous or next page) and (4) overall coverage of the

page. Speci#city ensures that the choice of label is not ambiguous,

while coverage ensures that all meaningful items on a p age can

we distributed the annotation workload and performed continuous

be annotated. We refrained from class labels that are very speci#c

quality controls. Phase one and two required a small team of experts

to a document category, such as *Abstract* in t he *Scienti!c A rticles*

only. For phases three and four, a group of 40 dedicated annotators

category. We also avoided class labels that are tightly linked to the

were assembled and supervised.

semantics o f the t ext. Labels such as *Author* and *A"liation,* a s seen

Phase 1 : D ata selection and preparation. Our inclusion cri

in DocBank, are often only distinguishable by discriminating on

teria for documents were described in Section 3. A large e!ort went

Figure 4: Table 1 from the DocLayNet paper in the original PDF (A), as rendered Markdown (B)

3

into ensuring that all documents are free to use. The data sources https://arxiv.org/

and in JSON representation (C). Spanning table cells, such as the multi-column header ”triple inter

annotator mAP@0.5-0.95 (%)”, is repeated for each column in the Markdown representation (B),

which guarantees that every data point can be traced back to row and column headings only by its

grid coordinates in the table. In the JSON representation, the span information is reflected in the

## fields of each table cell (C).

## 9
