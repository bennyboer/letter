# Letter - Document layout engine

> This is the successor of the [Thaw prototype](https://github.com/bennyboer/thaw) to be written in Rust

## Idea

A document essentially is a hierarchical structure of data with common elements like headlines, paragraphs, images, tables, and so on.
Most layout engines fall to the idea that documents are a linear sequence of elements, which is not the case.

For example a book consists of chapters, which consist of sections, which consist of paragraphs, which consist of sentences, which consist of words, which consist of letters.
A table consists of rows, which consist of cells.
Also, a section may be a child of another section, which is called a subsection.
When having a subsection all headlines are by default smaller than those of the parent section.

In HTML they have several elements for headlines of different sizes, whereas in Letter there is only one.
In LaTeX there is a macro for a chapter, a section, a subsection and a subsubsection, but not below.
In Letter there is only one element for a section which you may nest to any depth you want.
Want a subsubsubsubsubsubsection? Why not.

## Process

- Parsing the input files (`*.lsc` for document structure and content, `*.lst` for styles, `*.lmd` for metadata and variables) to a document
- Laying out the document (see [here](layout/README.md))
- (optionally) Rendering the document (PDF, ...)
