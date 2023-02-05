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

## Typesetting Process

- Parsing the input file (`*.lsc`)
- Building the document structure (or tree)
- Layouting the document (multiple passes if necessary to determine forward references for example)
  - Visit each element in the document tree
  - Ignore elements that are not visible and are just for defining the structure of the document (section, paragraph, etc.)
  - Layout the visible elements locally using constraints in the current context (e.g. the current page and the available space)
    - Text
    - Images
    - Tables
    - Lists
    - ...
  - Adjust the current context - e. g. lower the available space by the height/width of the element
  - When having a placeholder for a forward reference, replace it with the actual element (if known in the first pass)
  - If not known, schedule another pass after the current one
  - Schedule passes until the layout does not change anymore (or a maximum number of passes is reached)
- Rendering the document (PDF, ...)
