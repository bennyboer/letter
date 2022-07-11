# Letter script format

The `script` subproject of the Letter document layout engine contains the definition and parser for the Letter script markup language.
It is used to define the contents and layout of a Letter document and inherited mostly from XML/HTML and Markdown.
The idea is to be able to use something like Markdown to write your document contents while still being able to define details as extensively as with XML/HTML.

## Why both layout and content?

For this project we wanted to be able to separate layout and content as is convenient.
Modern UI tools seem to use a XML format solely for layout while the content is defined by source code.
While some seem to like it, other do not.
For Letter you won't be forced to separate layout and content.
Thus you define both in the Letter script format.
For example you might want to define the crude layout in a "main" .lsc file like the following:

```html
<document>

    <title> My document title </title>
    
    <abstract src="./abstract/de.lsc" lang="de"></abstract>
    <abstract src="./abstract/en.lsc" lang="en"</abstract>
    
    <table-of-contents></table-of-contents>
    
    <!-- Including all sections of the document in separate letter script files -->
    <section src="./contents/introduction.lsc"></section>
    <section src="./contents/basics.lsc"></section>
    <section src="./contents/methods.lsc"></section>
    <section src="./contents/results.lsc"></section>
    <section src="./contents/discussion.lsc"></section>
    <section src="./contents/summary.lsc"></section>

    <bibliography></bibliography>
    
</document>
```

In the above example the crude layout of a scientific document is depicted where a title is given, a table of contents, some content sections filled from other letter script files and a bibliography at the end.
For example the file `introduction.lsc` would then be something like the following:

```
# Introduction

My introduction text with simple Markdown syntax like having **bold** or *italic* text.
Optionally you can still use the HTML/XML like syntax to format text like <b>bold</b> or <i>italic</i> text.

<section>
    Alternatively you can also use a second-level heading using markdown with # or simply by using the `<header>` element.

    # My second level header

    is equivalent to

    <header>My second level header</header>

    This is a subsection paragraph!
</section>
```
