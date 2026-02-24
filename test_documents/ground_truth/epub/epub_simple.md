MathML

MathML

[REQUIRED]mathml-010Rendering

Tests whether MathML equation rendering is supported.

∫−∞∞e−x2dx=π∑n=1∞1n2=π26x=−b±b2−4ac2a

If the preceding equations are not presented as linear text (e.g., x=-b±b2-4ac2a),
    				the test passes.

[OPTIONAL]mathml-020CSS Styling of themathelement

Tests whether basic CSS styling of MathML is supported on themathelement.

2⁡x+y-z

The test passes if the equation has a yellow background and a dashed border.

If the reading system does not have a viewport, or does not support
    				CSS styles, this test should be markedNot Supported.

[OPTIONAL]mathml-021CSS Styling of themoelement

Tests whether basic CSS styling of MathML is supported on themoelement.

2⁡x+y-z

The test passes if the operators are enlarged relative to the other symbols and numbers.

If the reading system does not have a viewport, or does not support
                    CSS styles, this test should be markedNot Supported.

[OPTIONAL]mathml-022CSS Styling of themielement

Tests whether basic CSS styling of MathML is supported on themielement.

2⁡x+y-z

The test passes if the identifiers are bolded and blue.

If the reading system does not have a viewport, or does not support
                    CSS styles, this test should be markedNot Supported.

[OPTIONAL]mathml-023CSS Styling of themnelement

Tests whether basic CSS styling of MathML is supported on themnelement.

2⁡x+y-z

The test passes if the number 2 is italicized and blue.

If the reading system does not have a viewport, or does not support
                    CSS styles, this test should be markedNot Supported.

[REQUIRED]mathml-024Horizontal stretch,mover,munder, andmspaceelements

Tests whether horizontal stretch,mover,munder,mspaceelements are supported.

c=a⏟real+b⁢ⅈ⏟imaginary⏞complex number

The test passes if the rendering looks like .

[REQUIRED]mathml-025Testingmtablewithcolspanandrowspanattributes, Hebrew and Script fonts

Tests whethermtablewithcolspanandmspaceattributes (column and row spanning) are supported; uses Hebrew and Script alphabets.

covℒ⟶non𝒦⟶cof𝒦⟶cofℒ⟶2ℵ0↑↑↑↑𝔟⟶𝔡↑↑ℵ1⟶addℒ⟶add𝒦⟶cov𝒦⟶nonℒ

The test passes if the rendering looks likeCichoń's Diagram: .

[REQUIRED]mathml-026BiDi, RTL and Arabic alphabets

Tests whether right-to-left and Arabic alphabets are supported.

د⁡(س)={∑ٮ=1ص⁡سٮإذاكانس>0∫1ص⁡سٮ⁢ء⁡سإذاكانس∈مطا⁡πغيرذلك(معπ≃3,141)

The test passes if the rendering looks like the following image:

[REQUIRED]mathml-027Elementary math: long division notation

Tests whethermlongdivelements (from elementary math) are supported.

3435.313061210916151.091

The test passes if the rendering looks like the following image: .

Switch

epub:switch

[REQUIRED]switch-010Support

Tests whether theepub:switchelement is supported.

FAIL

PASS

If only the word "PASS" is rendered before this paragraph, the test passes. If both "PASS" and "FAIL" are rendered, or neither
        			"PASS" nor "FAIL" is rendered, the test fails.

[OPTIONAL]switch-020MathML Embedding

Tests whether the MathML namespace is recognized when used in anepub:caseelement.

2⁡x+y-z

FAIL

If a MathML equation is rendered before this paragraph, the test passes.

If testswitch-010did not pass, this test should be markedNot Supported.

EPUB 3.0 Compliance Test Suite

Reflowable EPUB 3 Conformance Test Document: 0100

Status of this Document

This publication is currently considered[UNDER DEVELOPMENT]by the IDPF.

This publication is part of versionX.Xof the EPUB 3.0 Compliance Test Suite released
    				onTBD.

Before using this publication to evaluate reading systems, testers are strongly encouraged to
	    			verify that they have the latest release by checking the current release version and date of
	    			the test suite atTBD

This publication is one of several that currently comprise the EPUB 3 conformance test suite
	    			for reflowable content. The complete test suite includes all of the following publications:

.

About this Document

This document focuses on human-evaluated binary (pass/fail) tests in a
	    			reflowable context. Tests for fixed-layout content and other individual tests that
	    			require a dedicated epub file are available in additional sibling documents; refer to
	    			thetest suite
	    				wiki(https://github.com/mgylling/epub-testsuite/wiki/Overview) for additional
	    			information.

Conventions

The following conventions are used throughout the document:

1. Locating a test

Tests forrequiredReading System functionality are
    							preceded by the label:[REQUIRED]

Tests foroptionalReading System functionality are
    							preceded by the label:[OPTIONAL]

2. Performing the testEach test includes a description of its purpose followed by the actualtest statement,
    					which can always be evaluated to true or false. These statements typically have the form:
    					"If [some condition], the test passes".3. Scoring in the results form@@@TODO provide info on where to get the results form
